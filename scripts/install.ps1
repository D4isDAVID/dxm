function Info {
    param (
        [Parameter(Mandatory, Position = 0)]
        [Object] $Object
    )

    Write-Host "info: $Object"
}

function Error {
    param (
        [Parameter(Mandatory, Position = 0)]
        [Object] $Object
    )

    [Console]::Error.WriteLine("error: $Object")
    exit 1
}

$downloadDir = Join-Path ([System.IO.Path]::GetTempPath()) (New-Guid)
New-Item -ItemType Directory -Path $downloadDir > $null

try {
    $program = "dxm"
    $repository = "D4isDAVID/dxm"

    $headers = @{
        "X-GitHub-Api-Version" = "2022-11-28"
    }

    if ($env:GITHUB_PAT) {
        Info "using provided GITHUB_PAT for authentication"
        $headers["Authorization"] = "token $env:GITHUB_PAT"
    }

    Info "fetching latest release"
    $apiUrl = "https://api.github.com/repos/$repository/releases/latest"

    try {
        $releaseJson = Invoke-RestMethod -Uri $apiUrl -Headers $headers -ErrorAction Stop
    } catch {
        Error "failed to fetch release data: $_"
    }

    $releaseTag = $releaseJson.tag_name
    $downloadUrl = "https://github.com/$repository/releases/download/$releaseTag/dxm-$releaseTag-windows-x64.zip"

    Info "downloading $downloadUrl"

    $archive = Join-Path $downloadDir "dxm.zip"
    try {
        Invoke-WebRequest $downloadUrl -OutFile $archive -Headers $headers -ErrorAction Stop
    } catch {
        Error "failed to download ${downloadUrl}: $_"
    }

    $binaryName = "$program.exe"
    Info "extracting $binaryName"

    try {
        Expand-Archive -Path $archive -DestinationPath $downloadDir -Force -ErrorAction Stop
    } catch {
        Error "failed to unarchive ${binaryName}: $_"
    }

    $binaryFile = Join-Path $downloadDir $binaryName

    Info "running $program"

    & $binaryFile self setup
} catch {
    Error "$_"
} finally {
    Remove-Item -Path $downloadDir -Recurse -Force -ErrorAction Ignore
}
