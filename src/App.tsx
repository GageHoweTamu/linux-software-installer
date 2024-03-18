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
  const [mode, setMode] = createSignal("left"); // mode for switchable windows

  const toggleMode = () => {
    setMode(mode() === "left" ? "right" : "left");
  };

  async function handleRunButtonClick() {
    try {
      setMessage("Loading...");
      const result = await open({ multiple: false });
      setMessage("done");
      const filename = Array.isArray(result) ? result[0] : result;
      const content = await invoke("read_file", { filename });
      setMessage(content as string);
    } catch (error) {
      console.error('Error:', error);
      setMessage("An error occurred: " + JSON.stringify(error));
    }
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
    <div>
      <button onClick={toggleMode}>Switch Mode</button>
      {mode() === "left" && (
        <div>
          <h1>Open a local InstallScript</h1>
          <button onClick={handleRunButtonClick}>Click me</button>
          <p>{message()}</p>
          <form
            onSubmit={(e) => {
              e.preventDefault();
              greet();
            }}
          >
            <input
              id="greet-input"
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder="..."
            />
            <button type="submit">buttonText</button>
          </form>
          <p>{greetMsg()}</p>
        </div>
      )}
      {mode() === "right" && (
        <div>
          <h2>Search the store</h2>
          <input type="search" id="search-input" placeholder="..." />
          {/* ... other right side content ... */}
        </div>
      )}
    </div>
 );
}

export default App;
