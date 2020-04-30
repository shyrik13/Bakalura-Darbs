Start-Process -FilePath "clang.exe"

$ProcessList = @(
    "clang" #or whatever you want to monitor
)

$properties=@(
	@{Name="Current Time"; Expression = {Get-Date -Format "dd-MM-yyyy HH:mm:ss"}},
    @{Name="Memory (MB)"; Expression = {[Math]::Round(($_.workingSetPrivate / 1mb),2)}}
)

Do {  
    $ProcessesFound = Get-Process | ? {$ProcessList -contains $_.Name} | Select-Object -ExpandProperty Name
    If ($ProcessesFound) {
		Get-WmiObject -class Win32_PerfFormattedData_PerfProc_Process -filter "Name='$($ProcessesFound)'" | 
			Select-Object $properties |
			Out-File -Encoding Ascii -append textfile.txt
		
        Start-Sleep 1
    }
} Until (!$ProcessesFound)