import { toast as showToast } from "react-toastify";

interface ToastProps {
  title: String;
  message: String;
}

function Toast({ title, message }: ToastProps) {
  return (
    <div className="flex w-full justify-between">
      <div className="flex flex-col">
        <span>{title}</span>
        <span className="text-content2">{message}</span>
      </div>
    </div>
  );
}

export const useToast = () => ({
  error(message: string) {
    showToast.error(<Toast title="Error" message={message} />);
  },
  success(message: string, title = "Success") {
    showToast.success(<Toast title={title} message={message} />);
  },
});
