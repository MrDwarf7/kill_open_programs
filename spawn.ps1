function spawn() {
    param(
        [int]$Count = $args
    )
    # programs array to pick from
    # $programs = @("notepad.exe", "mspaint.exe", "explorer.exe", "calc.exe")
    $programs = @("notepad.exe", "mspaint")

    for ($i = 0; $i -lt $Count; $i++) {
        $randomItem = Get-Random -InputObject $programs
        Start-Process $randomItem
    }
}

