import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { open } from "@tauri-apps/api/dialog";


function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  const [fileContent, setFileContent] = createSignal("");
  const [message, setMessage] = createSignal("");

  async function handleRunButtonClick() {
    console.log("Button clicked");
    const result = await open({ multiple: false });
    console.log(result);
    if (!result) {
      return;
      console.log("No file selected");
    }
    const filename = Array.isArray(result) ? result[0] : result;
    const content = await invoke("read_file", { filename });
    console.log(content);
    setMessage(content as string);
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  /* #[tauri::command]
  fn read_file(filename: &str) -> Result<String, std::io::Error> { // convert file content to string
      std::fs::read_to_string(filename)
  } */
  async function read_file() {
    setGreetMsg(await invoke("read_file", { file: fileContent() }));
    return fileContent();
  }

  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://solidjs.com" target="_blank">
          <img src={logo} class="logo solid" alt="Solid logo" />
        </a>
      </div>
        <div>
        <button onClick={handleRunButtonClick}>Click me</button>
        <p>{message()}</p>
      </div>

      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg()}</p>
    </div>
  );
}

export default App;
