use std::process::{Command, Stdio};
use tauri::command;

#[command]
pub async fn download_audio_mp3(youtube_url: String) -> Result<String, String> {
    // Caminho para o yt-dlp.exe
    let exe_path = "src-tauri/bin/yt-dlp.exe";
    // Pasta de saída dos áudios
    let output_dir = "audios";
    // Cria a pasta se não existir
    if let Err(e) = std::fs::create_dir_all(output_dir) {
        return Err(format!("Erro ao criar diretório de saída: {}", e));
    }
    // Monta o comando yt-dlp
    let output_template = format!("{}/%(artist)s - %(title)s [%(id)s].%(ext)s", output_dir);
    let args = [
        "-x", "--audio-format", "mp3",
        "--embed-metadata", "--embed-thumbnail", "--add-metadata",
        "--parse-metadata", "title:%(artist)s - %(title)s",
        "--output", &output_template,
        "--dump-json",
        &youtube_url
    ];
    let output = Command::new(exe_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Erro ao executar yt-dlp: {}", e))?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Erro yt-dlp: {}", err));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}
