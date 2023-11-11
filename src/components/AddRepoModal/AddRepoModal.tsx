import { useEffect, useState } from "react";
import { invoke } from "../../api";
import { useToast } from "../../hooks/useToast";

export interface AddRepoModalProps {
  show: boolean;
  onCancel: () => void;
  onConfirmComplete: () => void;
}

export default function AddRepoModal({
  show,
  onCancel,
  onConfirmComplete,
}: AddRepoModalProps) {
  const [loading, setLoading] = useState(false);
  const [url, setUrl] = useState("");
  const toast = useToast();

  useEffect(() => {
    if (show) return;
    // reset
    setLoading(false);
    setUrl("");
  }, [show]);

  const onConfirm = () => {
    setLoading(true);
    invoke("add_repository", { url })
      .then(onConfirmComplete)
      .catch((e) => {
        toast.error(e);
        onCancel();
      });
  };

  return (
    <>
      <input
        className="modal-state"
        id="modal-1"
        type="checkbox"
        checked={show}
        readOnly={true}
      />
      <div className="modal">
        <label
          className="modal-overlay"
          htmlFor="modal-1"
          onClick={onCancel}
        ></label>
        <div className="modal-content flex flex-col gap-5">
          <label
            htmlFor="modal-1"
            className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
            aria-roledescription="button"
            onClick={onCancel}
          >
            ✕
          </label>
          <h2 className="text-xl mb-4">Add Image Repository</h2>
          <input
            className="input input-solid w-full max-w-none"
            placeholder="https://github.com/url/to/repository.json"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
          />
          <span>
            Paste a URL to an image repository JSON file.{" "}
            <a className="link" href="https://github.com/mrjones2014/jdt">
              Click here to learn more.
            </a>
          </span>
          <div className="flex gap-3">
            <button className="btn btn-error btn-block" onClick={onCancel}>
              Cancel
            </button>
            <button className="btn btn-block" onClick={onConfirm}>
              {(loading && <div className="spinner-dot-intermittent" />) ||
                "Add"}
            </button>
          </div>
        </div>
      </div>
    </>
  );
}
