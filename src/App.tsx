import { useEffect, useState } from "react";
import { invoke } from "./api";
import "./App.css";
import { RepositoryViewModel } from "./types";

function App() {
  const [repos, setRepos] = useState<RepositoryViewModel[]>();
  useEffect(() => {
    invoke("get_repositories_view_model").then((repos) => {
      console.log("bruh?");
      setRepos(repos);
    });
  }, []);

  return JSON.stringify(repos, undefined, 2);
}

export default App;
