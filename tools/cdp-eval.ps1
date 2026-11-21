# Evaluate one JavaScript expression in the running panel through WebView2's CDP.
# Requires the app to be started with WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223
param(
    [Parameter(Mandatory = $true)][string]$Expression,
    [int]$Port = 9223,
    [int]$TimeoutSeconds = 10
)
$ErrorActionPreference = 'Stop'
$targets = Invoke-RestMethod "http://127.0.0.1:$Port/json/list" -TimeoutSec 5
$page = $targets | Where-Object { $_.type -eq 'page' } | Select-Object -First 1
if (-not $page) { throw 'no page target' }

$ws = [System.Net.WebSockets.ClientWebSocket]::new()
$cts = [System.Threading.CancellationTokenSource]::new([TimeSpan]::FromSeconds($TimeoutSeconds))
$ws.ConnectAsync([Uri]$page.webSocketDebuggerUrl, $cts.Token).GetAwaiter().GetResult()

$payload = @{
    id = 1
    method = 'Runtime.evaluate'
    params = @{ expression = $Expression; returnByValue = $true; awaitPromise = $true }
} | ConvertTo-Json -Depth 8 -Compress
$bytes = [System.Text.Encoding]::UTF8.GetBytes($payload)
[void]$ws.SendAsync([ArraySegment[byte]]::new($bytes), [System.Net.WebSockets.WebSocketMessageType]::Text, $true, $cts.Token).GetAwaiter().GetResult()

$buffer = New-Object byte[] 65536
$builder = [System.Text.StringBuilder]::new()
$reply = $null
while (-not $reply) {
    $result = $ws.ReceiveAsync([ArraySegment[byte]]::new($buffer), $cts.Token).GetAwaiter().GetResult()
    [void]$builder.Append([System.Text.Encoding]::UTF8.GetString($buffer, 0, $result.Count))
    if ($result.EndOfMessage) {
        $message = $builder.ToString() | ConvertFrom-Json
        [void]$builder.Clear()
        if ($message.id -eq 1) { $reply = $message }
    }
}
[void]$ws.CloseAsync([System.Net.WebSockets.WebSocketCloseStatus]::NormalClosure, 'done', [Threading.CancellationToken]::None).Wait()

if ($reply.error) { throw ($reply.error | ConvertTo-Json -Compress) }
$value = $reply.result.result.value
if ($null -eq $value) { $reply.result.result | ConvertTo-Json -Depth 6 } else { $value }
