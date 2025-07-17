import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [youtubeUrl, setYoutubeUrl] = useState("");
  const [statusMsg, setStatusMsg] = useState("");
  const [loading, setLoading] = useState(false);

  async function downloadAudioMp3() {
    setStatusMsg("");
    setLoading(true);
    try {
      // Chama o comando Tauri para baixar o áudio
      const result = await invoke<string>("download_audio_mp3", { youtubeUrl });
      setStatusMsg("Download concluído! Metadados:\n" + result);
    } catch (err: any) {
      setStatusMsg("Erro ao baixar: " + (err?.toString() ?? "Erro desconhecido"));
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="container">
      <h1>yt-dlrs - Baixar áudio do YouTube</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          if (youtubeUrl.trim()) downloadAudioMp3();
        }}
      >
        <input
          id="youtube-url-input"
          onChange={(e) => setYoutubeUrl(e.currentTarget.value)}
          placeholder="Cole a URL do YouTube aqui..."
          value={youtubeUrl}
          disabled={loading}
        />
        <button type="submit" disabled={loading || !youtubeUrl.trim()}>
          {loading ? "Baixando..." : "Download"}
        </button>
      </form>
      <pre style={{ whiteSpace: "pre-wrap" }}>{statusMsg}</pre>
    </main>
  );
}

export default App;
