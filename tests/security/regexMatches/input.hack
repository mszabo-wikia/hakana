use namespace HH\Lib\{Regex};

$url = HH\global_get('_GET')['url'];

if (Regex\matches($url, re"/^https://www.google.com/")) {
    $ch = curl_init($url);
}

if (Regex\matches($url, re"/^foo[0-9]*$/")) {
    echo $url;
}
