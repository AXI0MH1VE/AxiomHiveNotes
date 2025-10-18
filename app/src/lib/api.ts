import { invoke } from '@tauri-apps/api/core';

export interface Note {
  id?: number;
  title: string;
  content_md?: string;
  content_json?: string;
  folder: string;
  created_at?: string;
  updated_at?: string;
  reminder_at?: string;
  pinned: boolean;
  archived: boolean;
  deleted_at?: string;
  brand: string;
  backlinks_json: string;
  tags_cache: string;
}

export interface Tag {
  id?: number;
  name: string;
  created_at?: string;
}

export interface SearchResult {
  id: number;
  title: string;
  snippet: string;
  rank: number;
}

export const api = {
  // Notes
  createNote: (title: string, content_md: string, folder: string, brand: string) =>
    invoke<Note>('create_note', { title, content_md, folder, brand }),
  
  getNote: (id: number) =>
    invoke<Note>('get_note', { id }),
  
  updateNote: (
    id: number,
    title?: string,
    content_md?: string,
    folder?: string,
    pinned?: boolean,
    archived?: boolean
  ) =>
    invoke<Note>('update_note', { id, title, content_md, folder, pinned, archived }),
  
  deleteNote: (id: number) =>
    invoke('delete_note', { id }),
  
  listNotes: (
    folder?: string,
    pinned?: boolean,
    archived?: boolean,
    limit?: number,
    offset?: number
  ) =>
    invoke<Note[]>('list_notes', { folder, pinned, archived, limit, offset }),
  
  searchNotes: (query: string, limit?: number, offset?: number) =>
    invoke<SearchResult[]>('search_notes', { query, limit, offset }),
  
  // Tags
  createTag: (name: string) =>
    invoke<Tag>('create_tag', { name }),
  
  listTags: () =>
    invoke<Tag[]>('list_tags'),
  
  assignTags: (note_id: number, tag_ids: number[]) =>
    invoke('assign_tags', { note_id, tag_ids }),
  
  // Folders
  listFolders: () =>
    invoke<string[]>('list_folders'),
  
  // Settings
  getSetting: (key: string) =>
    invoke<string | null>('get_setting', { key }),
  
  setSetting: (key: string, value: string) =>
    invoke('set_setting', { key, value }),
  
  // Backlinks & Graph
  getBacklinks: (note_id: number) =>
    invoke<Array<[number, string]>>('get_backlinks', { note_id }),
  
  getGraphData: () =>
    invoke<{ nodes: Array<{ id: number; title: string }>; links: Array<{ source: number; target: number }> }>('get_graph_data'),
};
