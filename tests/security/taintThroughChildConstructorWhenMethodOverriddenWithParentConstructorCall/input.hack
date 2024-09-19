abstract class A {
    private string $taint;

    public function __construct($taint) {
        $this->taint = $taint;
    }

    public function getTaint() : string {
        return $this->taint;
    }
}

final class B extends A {
    public function __construct($taint) {
        parent::__construct($taint);
    }
}

$b = new B(HH\global_get('_GET')["bar"]);
echo $b->getTaint();