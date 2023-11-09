// TODO figure out a way to auto generate this file

import { invoke as tauriInvoke } from "@tauri-apps/api";
import { CommandRequest, CommandResponse } from "./types";

export function invoke(cmd: CommandRequest): Promise<CommandResponse> {
	return tauriInvoke("invoke", cmd);
}
