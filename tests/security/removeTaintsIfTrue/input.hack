function is_cool(
    <<Hakana\SecurityAnalysis\RemoveTaintsWhenReturningTrue('HtmlTag')>>
    string $s
): bool {
    return $s === "cool";
}

function foo(): void {
    $a = HH\global_get('_GET')['a'];

    if (is_cool($a)) {
        echo $a;
    }
}