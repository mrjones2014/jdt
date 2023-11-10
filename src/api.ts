// TODO figure out a way to auto generate this file

import { invoke as tauriInvoke } from "@tauri-apps/api";
import { RepositoryViewModel } from "./types";

type Invocations = {
	get_repositories_view_model: {
		args: undefined;
		returns: RepositoryViewModel[];
	};
	add_repository: {
		args: { url: string };
		returns: RepositoryViewModel;
	};
	delete_resource: {
		args: { path: string };
		returns: void;
	};
	update_repo: {
		args: { repo: RepositoryViewModel };
		returns: void;
	};
};

export function invoke<T extends keyof Invocations>(
	cmd: T,
	// makes the argument optional if the type is specified as void/undefined
	...args: undefined extends Invocations[T]["args"]
		? [undefined?]
		: [Invocations[T]["args"]]
): Promise<Invocations[T]["returns"]> {
	return tauriInvoke(cmd, args[0] ?? {});
}
