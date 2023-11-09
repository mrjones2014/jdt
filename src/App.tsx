import { useEffect, useState } from "react";
import { invoke } from "./api";
import { RepositoryViewModel } from "./types";
import AddRepoModal from "./components/AddRepoModal/AddRepoModal";

function formatUrl(url: string): string {
  let formatted = url.substring(17);
  if (formatted.length < url.length) {
    formatted = `${formatted}…`;
  }
  return formatted;
}

function App() {
  const [repos, setRepos] = useState<RepositoryViewModel[]>();
  useEffect(() => {
    invoke("get_repositories_view_model")
      .then((repos) => {
        console.log("bruh?");
        setRepos(repos);
      })
      .catch(() => {
        // TODO error toasts
      });
  }, []);

  const [showAddModal, setShowAddModal] = useState(false);

  const onRepoAdded = () => {
    setShowAddModal(false);
    invoke("get_repositories_view_model").then((repos) => {
      console.log("bruh?");
      setRepos(repos);
    });
  };

  return (
    <div>
      <div className="md:flex md:items-center md:justify-between mb-4">
        <h1 className="text-2xl font-bold leading-7">
          Installed Image Repositories
        </h1>
        <button
          className="btn btn-primary"
          onClick={() => setShowAddModal(true)}
        >
          Add Image Repository
        </button>
      </div>
      <div className="flex w-full overflow-x-auto">
        <table className="table zebra-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
              <th>Path</th>
              <th>Update URL</th>
              <th>Last Updated</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {(repos &&
              repos.map((repo) => (
                <tr>
                  <td>{repo.name}</td>
                  <td>{repo.description}</td>
                  <td>
                    {(repo.updateUrl && (
                      <span
                        className="tooltip tooltip-top"
                        data-tooltip={repo.updateUrl}
                      >
                        {formatUrl(repo.updateUrl)}
                      </span>
                    )) ||
                      "None"}
                  </td>
                  <td>{repo.lastUpdated}</td>
                  <td>todo</td>
                </tr>
              ))) || (
              <tr>
                <td colSpan={6}>No Repositories Added</td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
      <AddRepoModal
        show={showAddModal}
        onCancel={() => setShowAddModal(false)}
        onConfirmComplete={onRepoAdded}
      />
    </div>
  );
}

export default App;
