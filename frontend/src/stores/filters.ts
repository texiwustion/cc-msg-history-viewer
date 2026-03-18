import { createSignal } from 'solid-js'

export const [selectedProject, setSelectedProject] = createSignal<string | undefined>(undefined)
export const [selectedSession, setSelectedSession] = createSignal<string | undefined>(undefined)
export const [searchQuery, setSearchQuery] = createSignal('')
export const [fromTs, setFromTs] = createSignal<number | undefined>(undefined)
export const [toTs, setToTs] = createSignal<number | undefined>(undefined)
