use std::process::{Command, Stdio};
use tauri::command;

#[command]
pub async fn download_audio_mp3(youtube_url: String) -> Result<String, String> {
    // Caminho robusto para o yt-dlp.exe usando CARGO_MANIFEST_DIR
    // Define o diretório base do app
    let home_dir = dirs::home_dir().ok_or_else(|| "Não foi possível localizar o diretório home do usuário.".to_string())?;
    let app_dir = home_dir.join(".yt-dlrs");
    let bin_dir = app_dir.join("bin");
    // Obtém o diretório de músicas do Windows
    let music_dir = dirs::audio_dir().ok_or_else(|| "Não foi possível localizar a pasta de músicas do usuário.".to_string())?;
    // Cria as pastas se não existirem
    if let Err(e) = std::fs::create_dir_all(&bin_dir) {
        return Err(format!("Erro ao criar diretório de binários: {}", e));
    }
    if let Err(e) = std::fs::create_dir_all(&music_dir) {
        return Err(format!("Erro ao criar diretório de músicas: {}", e));
    }
    // Caminho do yt-dlp.exe
    let exe_path = bin_dir.join("yt-dlp.exe");
    // Monta o comando yt-dlp
    // Template apenas do nome do arquivo, sem caminho absoluto
    let output_template = "%(artist)s - %(title)s [%(id)s].%(ext)s";
    let args = [
        "-x", "--audio-format", "mp3",
        "--embed-metadata", "--embed-thumbnail", "--add-metadata",
        "--parse-metadata", "title:%(artist)s - %(title)s",
        "--output", output_template,
        &youtube_url
    ];
    // Logging para depuração
    println!("[yt-dlrs] Caminho yt-dlp.exe: {}", exe_path.display());
    println!("[yt-dlrs] Pasta de saída: {}", music_dir.display());
    println!("[yt-dlrs] Output template: {}", output_template);
    println!("[yt-dlrs] Args: {:?}", args);

    let output = Command::new(exe_path.to_str().unwrap())
        .args(&args)
        .current_dir(&music_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Erro ao executar yt-dlp: {}", e))?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("[yt-dlrs] STDERR yt-dlp: {}", err);
        return Err(format!("Erro yt-dlp: {}", err));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("[yt-dlrs] STDOUT yt-dlp: {}", stdout);
    Ok(stdout.to_string())
}
