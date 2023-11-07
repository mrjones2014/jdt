import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { ImageRepo } from "./types";

function App() {
  const [repos, setRepos] = useState<ImageRepo[]>();
  useEffect(() => {
    // TODO this API sucks. Figure out a way to make invocations typesafe.
    invoke("get_repositories_viewmodel").then((repos) => {
      setRepos(repos as ImageRepo[]);
    });
  });

  return JSON.stringify(repos, undefined, 2);
}

export default App;
