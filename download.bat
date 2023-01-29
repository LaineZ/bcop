set FILE=sciter-js-sdk-4.4.9.3.zip

if not exist %FILE% (
    curl https://gitlab.com/sciter-engine/sciter-js-sdk/-/archive/4.4.9.3/sciter-js-sdk-4.4.9.3.zip --output %FILE%
)

powershell Expand-Archive %FILE% -DestinationPath .