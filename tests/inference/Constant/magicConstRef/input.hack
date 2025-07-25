use namespace HH\Lib\{Dict, Vec};

final class Codegen {
    const string FOO = 'foo';
    public static function forPath(string $foo): this {}
    public function bar(): void {}
}

<<__EntryPoint>>
async function local_create_experiment_schema(): Awaitable<void> {
    await \HH\Asio\usleep(100000);
	Codegen::forPath(__DIR__)->bar(Codegen::FOO);
}
