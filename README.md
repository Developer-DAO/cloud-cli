# D_D Cloud CLI 

Conveniently manage API keys, track service usage, and view account balances all from your terminal**

<ins>Usage:</ins> dd-cloud <**COMMAND**>

|<ins>Commands:</ins>| <ins> Description </ins>|
|:--------------:|:---------------------------------------------------------------:|
|get-api-key | Retrieve one of your API key for D_D Cloud|
|delete-api-key| Delete one of your API keys for D_D Cloud|
|new-api-key| Generate a new API key for D_D Cloud. Max 10|
|track-usage| Returns the number of calls you made this cycle|
|balance| Returns your account balance
|help| Print this message or the help of the given subcommand(s)|
------------------------------------------------------------------------------------

Get API Key has an optional flag called `--unsafe-print` that permits the user to print their API key to stdout. This command flushes stdout before printing the API key, so the output can be piped into other commands.
For more info, try `dd-cloud get-api-key --help`

New API Key has an optional argument invoked by `--secret-manager` or `-s` that allows users to store their newly generated API key directly in a secret storage service such as AWS Secret Manager. More services will be supported in the future. 
The only current option for `--secret-manager` or `-s` is `--aws`. For more info, try `dd-cloud new-api-key --help`. 
