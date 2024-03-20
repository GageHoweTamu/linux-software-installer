import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { open } from "@tauri-apps/api/dialog";


function App() {
  const [greetMsg, setGreetMsg] = createSignal("");

  const [name, setName] = createSignal("");

  const [fileContent, setFileContent] = createSignal(""); // the content of the file

  const [scriptPreview, setScriptPreview] = createSignal("");
  const [scriptOutput, setScriptOutput] = createSignal("");

  const [mode, setMode] = createSignal("left"); // mode for switchable windows
  const toggleMode = () => {
    setMode(mode() === "left" ? "right" : "left");
  };

  const [filePath, setfilePath] = createSignal("/home/kali/Downloads/test2.uinstall"); // file path for install script

  async function handleRunButtonClick() {
    try {
      setScriptPreview("Loading...");
      const result = await open({ multiple: false });
      setScriptPreview("done");
      const filename = Array.isArray(result) ? result[0] : result;
      const content = await invoke("read_file", { filename });
      const returned = await invoke("run_bash", { filepath: filename });
      setScriptPreview(content as string);
      setScriptOutput(returned as string);
      if (filename) {
        setfilePath(filename);
      } else {
        console.error('No file selected');
      }
    } catch (error) {
      console.error('Error:', error);
      setScriptPreview("An error occurred: " + JSON.stringify(error));
    }
  }

  async function read_file() {
    setGreetMsg(await invoke("read_file", { file: fileContent() }));
    return fileContent();
  }
  async function run_bash() {
    setGreetMsg(await invoke("run_script", { file: fileContent() }));
    return fileContent();
  }

  return (
    <div>
      <button onClick={toggleMode}>Switch Mode</button>
      {mode() === "left" && (
        <div>
          <h1>Open a local InstallScript</h1>
          <button onClick={handleRunButtonClick}>Load .uinstall file</button>
          <p>{scriptPreview()}</p>
          <form
            onSubmit={(e) => {
              e.preventDefault();
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
          <p>{scriptOutput()}</p>
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
