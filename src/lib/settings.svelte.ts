// the long names are super ugly, but i like the software so it's fine
import { createTauriRevisionSync as tauri_sync, createTauriFileBackend as tauri_storage } from '@statesync/tauri';
import { loadPersistedSnapshot, createPersistenceApplier as persistant_applier } from "@statesync/persistence";

import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

import { type Settings } from './types/settings';
import type { RevisionSyncHandle } from '@statesync/core';

function deepEqual(a: any, b: any): boolean {
    if (a === b) return true;
    if (a == null || b == null) return a === b;
    if (typeof a !== 'object' || typeof b !== 'object') return false;

    const keysA = Object.keys(a);
    const keysB = Object.keys(b);

    if (keysA.length !== keysB.length) return false;

    return keysA.every(key => deepEqual(a[key], b[key])) && keysB.every(key => key in a);
}

class SettingsState {
    current = $state<Settings>({} as unknown as Settings)
    has_init = $state(false);
    private last_snapshot: Settings | null = null

    private sync: RevisionSyncHandle;

    constructor() {
        // makes typescript shut up about `this`
        const self = this;
        const storage = tauri_storage<Settings>({
            invoke,
            loadCommand: "load_settings",
            saveCommand: "save_settings"
        })
        const applier = persistant_applier<Settings>({
            applier: {
                apply(snapshot) {
                    self.has_init = true
                    self.last_snapshot = snapshot.data
                    Object.assign(self.current as object, snapshot.data)
                }
            },
            storage,
        })

        this.sync = tauri_sync<Settings>({
            topic: 'settings',
            listen,
            invoke,
            eventName: 'settings:invalidated',
            commandName: 'get_settings',
            applier
        })

        loadPersistedSnapshot(storage, applier).then(current => {
            if(current) this.current = current.data
        })
    }

    async init(): Promise<null> {
        const { promise, resolve } = Promise.withResolvers<null>()

        $effect(() => {
            if (!this.has_init) return
            resolve(null)
            if (deepEqual(this.current, this.last_snapshot)) return;
            invoke("update_settings", {
                settings: this.current
            })
        })
        await this.sync.start()
        return promise;
    }

}

export default new SettingsState();
