# Generate Solana Keypair Secret for GitHub Actions

Write-Host "Generating Solana Keypair Secret for GitHub Actions..." -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan

# Check if keypair exists
$keypairPath = "$env:USERPROFILE\.config\solana\id.json"
if (-Not (Test-Path $keypairPath)) {
    Write-Host "No Solana keypair found at: $keypairPath" -ForegroundColor Red
    Write-Host "Please run solana-keygen new first to generate a keypair." -ForegroundColor Yellow
    exit 1
}

Write-Host "Found keypair at: $keypairPath" -ForegroundColor Green

# Convert to base64
try {
    $keypairBytes = Get-Content $keypairPath -Raw -Encoding UTF8
    $base64Keypair = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($keypairBytes))
    
    Write-Host "`nBase64 Encoded Keypair:" -ForegroundColor Green
    Write-Host "=======================================================" -ForegroundColor Cyan
    Write-Host $base64Keypair -ForegroundColor White
    Write-Host "=======================================================" -ForegroundColor Cyan
    
    Write-Host "`nInstructions:" -ForegroundColor Yellow
    Write-Host "1. Copy the base64 string above" -ForegroundColor White
    Write-Host "2. Go to your GitHub repository" -ForegroundColor White
    Write-Host "3. Navigate to Settings - Secrets and variables - Actions" -ForegroundColor White
    Write-Host "4. Click New repository secret" -ForegroundColor White
    Write-Host "5. Name: SOLANA_KEYPAIR" -ForegroundColor White
    Write-Host "6. Value: Paste the base64 string" -ForegroundColor White
    Write-Host "7. Click Add secret" -ForegroundColor White
    
    Write-Host "`nSecurity Note:" -ForegroundColor Red
    Write-Host "Keep your keypair secure and never share it publicly!" -ForegroundColor Red
    Write-Host "This secret will be used by GitHub Actions to deploy your program." -ForegroundColor Yellow
    
} catch {
    Write-Host "Failed to convert keypair to base64: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host "`nKeypair secret generation completed!" -ForegroundColor Green
