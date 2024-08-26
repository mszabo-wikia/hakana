use hakana_logger::Logger;
use hakana_code_info::analysis_result::AnalysisResult;
use hakana_code_info::code_location::FilePath;
use hakana_code_info::codebase_info::CodebaseInfo;
use hakana_code_info::diff::CodebaseDiff;
use hakana_code_info::issue::Issue;
use hakana_code_info::symbol_references::SymbolReferences;
use hakana_str::Interner;
use hakana_str::StrId;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

use crate::cache::load_cached_existing_issues;
use crate::cache::load_cached_existing_references;

#[derive(Default)]
pub(crate) struct CachedAnalysis {
    pub safe_symbols: FxHashSet<StrId>,
    pub safe_symbol_members: FxHashSet<(StrId, StrId)>,
    pub existing_issues: FxHashMap<FilePath, Vec<Issue>>,
    pub symbol_references: SymbolReferences,
}

pub(crate) fn mark_safe_symbols_from_diff(
    logger: &Logger,
    codebase_diff: CodebaseDiff,
    codebase: &CodebaseInfo,
    interner: &mut Interner,
    invalid_scanned_files: FxHashSet<FilePath>,
    files_to_analyze: &mut Vec<String>,
    issues_path: &Option<String>,
    references_path: &Option<String>,
    previous_analysis_result: Option<AnalysisResult>,
) -> CachedAnalysis {
    let (existing_references, mut existing_issues) = if let Some(previous_analysis_result) =
        previous_analysis_result
    {
        (
            previous_analysis_result.symbol_references,
            previous_analysis_result.emitted_issues,
        )
    } else if let (Some(issues_path), Some(references_path)) = (issues_path, references_path) {
        let existing_references = if let Some(existing_references) =
            load_cached_existing_references(references_path, true, logger)
        {
            existing_references
        } else {
            return CachedAnalysis::default();
        };

        let existing_issues =
            if let Some(existing_issues) = load_cached_existing_issues(issues_path, true, logger) {
                existing_issues
            } else {
                return CachedAnalysis::default();
            };

        (existing_references, existing_issues)
    } else {
        return CachedAnalysis::default();
    };

    let (invalid_symbols_and_members, partially_invalid_symbols) =
        if let Some(invalid_symbols) = existing_references.get_invalid_symbols(&codebase_diff) {
            invalid_symbols
        } else {
            // this happens when there are too many invalidated symbols
            return CachedAnalysis::default();
        };

    let mut cached_analysis = CachedAnalysis {
        symbol_references: existing_references,
        ..CachedAnalysis::default()
    };

    for keep_symbol in &codebase_diff.keep {
        if !invalid_symbols_and_members.contains(keep_symbol) {
            if keep_symbol.1.is_empty() {
                if !partially_invalid_symbols.contains(&keep_symbol.0) {
                    cached_analysis.safe_symbols.insert(keep_symbol.0);
                }
            } else {
                cached_analysis
                    .safe_symbol_members
                    .insert((keep_symbol.0, keep_symbol.1));
            }
        }
    }

    cached_analysis
        .symbol_references
        .remove_references_from_invalid_symbols(&invalid_symbols_and_members);

    let mut invalid_files = codebase
        .files
        .iter()
        .filter(|(_, file_info)| {
            file_info.ast_nodes.iter().any(|node| {
                invalid_symbols_and_members.contains(&(node.name, StrId::EMPTY))
                    || partially_invalid_symbols.contains(&node.name)
            })
        })
        .map(|(file_id, _)| interner.lookup(&file_id.0))
        .collect::<FxHashSet<_>>();

    invalid_files.extend(
        invalid_scanned_files
            .iter()
            .map(|file_id| interner.lookup(&file_id.0)),
    );

    files_to_analyze.retain(|full_path| invalid_files.contains(&full_path.as_str()));

    update_issues_from_diff(
        &mut existing_issues,
        codebase_diff,
        &invalid_symbols_and_members,
    );
    cached_analysis.existing_issues = existing_issues;

    cached_analysis
}

fn update_issues_from_diff(
    existing_issues: &mut FxHashMap<FilePath, Vec<Issue>>,
    codebase_diff: CodebaseDiff,
    invalid_symbols_and_members: &FxHashSet<(StrId, StrId)>,
) {
    for (existing_file, file_issues) in existing_issues.iter_mut() {
        file_issues.retain(|issue| {
            !invalid_symbols_and_members.contains(&issue.symbol)
                && issue.symbol.0 != existing_file.0
        });

        if file_issues.is_empty() {
            continue;
        }

        let diff_map = codebase_diff
            .diff_map
            .get(existing_file)
            .cloned()
            .unwrap_or(vec![]);

        let deletion_ranges = codebase_diff
            .deletion_ranges_map
            .get(existing_file)
            .cloned()
            .unwrap_or(vec![]);

        if !deletion_ranges.is_empty() {
            file_issues.retain(|issue| {
                for (from, to) in &deletion_ranges {
                    if &issue.pos.start_offset >= from && &issue.pos.start_offset <= to {
                        return false;
                    }
                }

                true
            });
        }

        if !diff_map.is_empty() {
            for issue in file_issues {
                for (from, to, file_offset, line_offset) in &diff_map {
                    if &issue.pos.start_offset >= from && &issue.pos.start_offset <= to {
                        issue.pos.start_offset =
                            ((issue.pos.start_offset as isize) + file_offset) as u32;
                        issue.pos.end_offset =
                            ((issue.pos.end_offset as isize) + file_offset) as u32;
                        issue.pos.start_line =
                            ((issue.pos.start_line as isize) + line_offset) as u32;
                        issue.pos.end_line = ((issue.pos.end_line as isize) + line_offset) as u32;
                        break;
                    }
                }
            }
        }
    }
}
