# Define the characters to use
$chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ 123456789".ToCharArray()

for ($j = 0; $j -lt 1000; $j++) {
    # Initialize an empty string
    $randomString = ""
    for ($i = 0; $i -lt 10000; $i++) {
        $randomChar = $chars | Get-Random
        $randomString += $randomChar
    }

    Set-Content -LiteralPath ./$j.txt -Value $randomString
}
