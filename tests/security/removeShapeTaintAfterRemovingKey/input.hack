function foo(): void {
    $a = HH\global_get('_GET')["a"];

    $b = dict["x" => $a, "y" => "hello"];

    Shapes::removeKey(inout $b, "x");

    $c = $b;

    foreach ($c as $i) {
        echo $i;
    }
}