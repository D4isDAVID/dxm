param (
    [Parameter()]
    [ValidateNotNullOrEmpty()]
    [string] $Release = "latest",

    [Parameter()]
    [ValidateNotNullOrEmpty()]
    [string] $InstallDir,

    [Parameter()]
    [switch] $NoEnvPath
)
begin {
    function Info {
        [CmdletBinding()]
        param (
            [Parameter(Mandatory, Position = 0)]
            [Object] $Object
        )
        process {
            Write-Host "info: $Object"
        }
    }


    function Error {
        [CmdletBinding()]
        param (
            [Parameter(Mandatory, Position = 0)]
            [Object] $Object
        )
        process {
            [Console]::Error.WriteLine("error: $Object")
            exit 1
        }
    }


    function Ensure-Dependencies {
        [CmdletBinding()]
        param (
            [Parameter(Mandatory, Position = 0)]
            [string[]] $Dependencies
        )
        process {
            foreach ($dep in $Dependencies) {
                if (!(Get-Command $dep -ErrorAction SilentlyContinue)) {
                    Error "'$dep' is not installed or available"
                }
            }
        }
    }


    $downloadDir = Join-Path ([System.IO.Path]::GetTempPath()) (New-Guid)
    New-Item -ItemType Directory -Path $downloadDir > $null

    $program = "dxm"
    $repository = "D4isDAVID/dxm"

    if ($IsWindows) {
        $os = "windows"

        $unarchiveCommand = "tar"
        $unarchiveOptions = "-C", "$downloadDir", "-xZf"
        $archiveExtension = "zip"
    } elseif ($IsLinux) {
        $os = "linux"

        $unarchiveCommand = "tar"
        $unarchiveOptions = "-C", "$downloadDir", "-xzf"
        $archiveExtension = "tar.gz"
    } else {
        Error "unsupported operating system '$os'"
    }

    Ensure-Dependencies $unarchiveCommand

    $arch = ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture).ToString().ToLower()
    if ($arch -ne "x64") {
        Error "unsupported architecture '$arch'"
    }

    $programArgs = @()

    if ($PSBoundParameters.ContainsKey('InstallDir')) {
        $env:DXM_HOME = $InstallDir
        [System.Environment]::SetEnvironmentVariable("DXM_HOME", $InstallDir, "User")
    }
    if ($NoEnvPath) {
        $programArgs.Add("--no-env-path")
    }

    $headers = @{
        "X-GitHub-Api-Version" = "2022-11-28"
    }

    if ($env:GITHUB_PAT) {
        Info "using provided GITHUB_PAT for authentication"
        $headers["Authorization"] = "token $env:GITHUB_PAT"
    }
}
process {
    if ($Release -eq "latest") {
        Info "fetching latest release"
        $apiUrl = "https://api.github.com/repos/$repository/releases/latest"
    } else {
        Info "fetching release $Release"
        $apiUrl = "https://api.github.com/repos/$repository/releases/tags/$Release"
    }

    try {
        $releaseJson = Invoke-RestMethod -Uri $apiUrl -Headers $headers -ErrorAction Stop
    } catch {
        Error "failed to fetch release data: $_"
    }

    $releaseTag = $releaseJson.tag_name
    $downloadUrl = "https://github.com/$repository/releases/download/$releaseTag/dxm-$releaseTag-$os-$arch.$archiveExtension"

    Info "downloading $downloadUrl"

    $archive = Join-Path $downloadDir "dxm.$archiveExtension"
    try {
        Invoke-WebRequest $downloadUrl -OutFile $archive -Headers $headers -ErrorAction Stop
    } catch {
        Error "failed to download ${downloadUrl}: $_"
    }

    $binaryName = $program
    if ($IsWindows) {
        $binaryName = "$binaryName.exe"
    }

    Info "extracting $binaryName"

    & $unarchiveCommand $unarchiveOptions $archive $binaryName
    if ($LASTEXITCODE -ne 0) {
        Error "failed to unarchive $binaryName"
    }

    $binaryFile = Join-Path $downloadDir $binaryName

    Info "running $program"

    if (!($IsWindows)) {
        & chmod +x $binaryFile
    }

    & $binaryFile self setup $programArgs
}
end {
    Remove-Item -Path $downloadDir -Recurse -Force -ErrorAction Ignore
}
