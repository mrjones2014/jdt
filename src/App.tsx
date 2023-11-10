import { useEffect, useState } from "react";
import { invoke } from "./api";
import { RepositoryViewModel } from "./types";
import AddRepoModal from "./components/AddRepoModal/AddRepoModal";
import { ArrowPathIcon, TrashIcon } from "@heroicons/react/20/solid";

function formatUrl(url: string): string {
  const urlParsed = new URL(url);
  return `${urlParsed.hostname}/...`;
}

function App() {
  const [repos, setRepos] = useState<RepositoryViewModel[]>();

  const refresh = () =>
    invoke("get_repositories_view_model")
      .then((repos) => {
        setRepos(repos);
      })
      .catch((e) => {
        console.error(e);
        // TODO error toasts
      });

  useEffect(() => {
    refresh();
  }, []);

  const [showAddModal, setShowAddModal] = useState(false);
  const [deletePending, setDeletePending] = useState(false);

  const onRepoAdded = () => {
    setShowAddModal(false);
    refresh();
  };

  const updateRepo = (repo: RepositoryViewModel) => {
    invoke("update_repo", { repo }).then(refresh).catch(console.error); // TODO error toast
  };

  const deleteRepo = (repo: RepositoryViewModel) => {
    if (!deletePending) {
      setDeletePending(true);
      setTimeout(() => {
        setDeletePending(false);
      }, 3000);
      return;
    }
    invoke("delete_resource", { path: repo.path })
      .then(refresh)
      .catch(console.error); // TODO error toasts
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
      <div className="flex w-full">
        <table className="table zebra-table table-hover table-compact">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
              <th>Update URL</th>
              <th>Last Updated</th>
              <th className="w-4">Actions</th>
            </tr>
          </thead>
          <tbody>
            {(repos &&
              repos.length > 0 &&
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
                        <a
                          className="link"
                          href={repo.updateUrl}
                          target="_blank"
                          rel="noopener noreferrer"
                        >
                          {formatUrl(repo.updateUrl)}
                        </a>
                      </span>
                    )) ||
                      "None"}
                  </td>
                  <td>{repo.lastUpdated}</td>
                  <td className="flex justify-end w-fit">
                    {repo.updateUrl && (
                      <button
                        className="btn btn-primary mr-4"
                        onClick={() => updateRepo(repo)}
                      >
                        <ArrowPathIcon className="h-6 w-6 text-white" />
                        &nbsp;&nbsp; Update
                      </button>
                    )}
                    <button
                      className="btn btn-error w-32"
                      onClick={() => deleteRepo(repo)}
                    >
                      <TrashIcon className="h-6 w-6 text-white" />{" "}
                      {(deletePending && "Confirm?") || "Delete"}
                    </button>
                  </td>
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
