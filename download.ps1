$file_sciter = "sciter-js-sdk-4.4.9.3.zip"
$file_bass = "bass24.zip"

if (-not(Test-Path -Path $file_sciter -PathType Leaf)) {
    Invoke-WebRequest -Uri "https://gitlab.com/sciter-engine/sciter-js-sdk/-/archive/4.4.9.3/sciter-js-sdk-4.4.9.3.zip" -OutFile $file_sciter
}

if (-not(Test-Path -Path $file_bass -PathType Leaf)) {
    Invoke-WebRequest -Uri "https://www.un4seen.com/files/bass24.zip" -OutFile $file_bass
}

Expand-Archive -Force $file_sciter '.'
New-Item -Path "bass24" -ItemType Directory
Expand-Archive -Force $file_bass './bass24'