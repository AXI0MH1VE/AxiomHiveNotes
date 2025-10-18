import { useEffect, useState } from 'react';
import { api, type Note } from './lib/api';
import { Plus, Search, Trash2, Pin, Archive } from 'lucide-react';

function App() {
  const [notes, setNotes] = useState<Note[]>([]);
  const [currentNote, setCurrentNote] = useState<Note | null>(null);
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    loadNotes();
  }, []);

  const loadNotes = async () => {
    try {
      const loadedNotes = await api.listNotes();
      setNotes(loadedNotes);
      if (loadedNotes.length > 0 && !currentNote) {
        setCurrentNote(loadedNotes[0]);
      }
    } catch (error) {
      console.error('Failed to load notes:', error);
    }
  };

  const createNewNote = async () => {
    try {
      const newNote = await api.createNote(
        'Untitled Note',
        '',
        '',
        '@AxiomHive'
      );
      setNotes([newNote, ...notes]);
      setCurrentNote(newNote);
    } catch (error) {
      console.error('Failed to create note:', error);
    }
  };

  const updateNote = async (updates: Partial<Note>) => {
    if (!currentNote?.id) return;
    
    try {
      const updated = await api.updateNote(
        currentNote.id,
        updates.title,
        updates.content_md,
        updates.folder,
        updates.pinned,
        updates.archived
      );
      setCurrentNote(updated);
      setNotes(notes.map(n => n.id === updated.id ? updated : n));
    } catch (error) {
      console.error('Failed to update note:', error);
    }
  };

  const deleteNote = async (id: number) => {
    try {
      await api.deleteNote(id);
      const newNotes = notes.filter(n => n.id !== id);
      setNotes(newNotes);
      if (currentNote?.id === id) {
        setCurrentNote(newNotes[0] || null);
      }
    } catch (error) {
      console.error('Failed to delete note:', error);
    }
  };

  const handleSearch = async (query: string) => {
    setSearchQuery(query);
    if (!query.trim()) {
      loadNotes();
      return;
    }
    
    try {
      const results = await api.searchNotes(query);
      const searchedNotes = await Promise.all(
        results.map(r => api.getNote(r.id))
      );
      setNotes(searchedNotes);
    } catch (error) {
      console.error('Search failed:', error);
      loadNotes();
    }
  };

  return (
    <div className="flex h-screen bg-gray-900 text-white">
      {/* Sidebar */}
      <div className="w-80 border-r border-gray-700 flex flex-col">
        <div className="p-4 border-b border-gray-700">
          <h1 className="text-2xl font-bold text-purple-500">@AxiomHive Notes</h1>
        </div>
        
        <div className="p-4 space-y-2">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
            <input
              type="text"
              placeholder="Search notes..."
              value={searchQuery}
              onChange={(e) => handleSearch(e.target.value)}
              className="w-full pl-10 pr-4 py-2 bg-gray-800 text-white rounded-md border border-gray-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            />
          </div>
          
          <button
            onClick={createNewNote}
            className="w-full flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition-colors"
          >
            <Plus className="w-4 h-4" />
            New Note
          </button>
        </div>
        
        <div className="flex-1 overflow-y-auto">
          {notes.map((note) => (
            <div
              key={note.id}
              onClick={() => setCurrentNote(note)}
              className={`p-4 border-b border-gray-700 cursor-pointer transition-colors ${
                currentNote?.id === note.id
                  ? 'bg-gray-800'
                  : 'hover:bg-gray-800/50'
              }`}
            >
              <div className="flex items-start justify-between gap-2">
                <h3 className="font-semibold truncate flex-1">{note.title}</h3>
                <div className="flex gap-1">
                  {note.pinned && <Pin className="w-3 h-3 text-purple-500" />}
                  {note.archived && <Archive className="w-3 h-3 text-gray-500" />}
                </div>
              </div>
              <p className="text-sm text-gray-400 truncate mt-1">
                {note.content_md || 'Empty note'}
              </p>
              <span className="text-xs text-gray-500 mt-1 block">
                {new Date(note.updated_at || '').toLocaleDateString()}
              </span>
            </div>
          ))}
        </div>
      </div>
      
      {/* Editor */}
      <div className="flex-1 flex flex-col">
        {currentNote ? (
          <>
            <div className="p-4 border-b border-gray-700 flex items-center justify-between">
              <input
                type="text"
                value={currentNote.title}
                onChange={(e) => {
                  const updated = { ...currentNote, title: e.target.value };
                  setCurrentNote(updated);
                }}
                onBlur={() => updateNote({ title: currentNote.title })}
                className="text-3xl font-bold bg-transparent border-none outline-none flex-1"
              />
              <div className="flex gap-2">
                <button
                  onClick={() => updateNote({ pinned: !currentNote.pinned })}
                  className={`p-2 rounded-md ${
                    currentNote.pinned
                      ? 'bg-purple-600 text-white'
                      : 'hover:bg-gray-800'
                  }`}
                >
                  <Pin className="w-5 h-5" />
                </button>
                <button
                  onClick={() => currentNote.id && deleteNote(currentNote.id)}
                  className="p-2 rounded-md hover:bg-red-600 hover:text-white transition-colors"
                >
                  <Trash2 className="w-5 h-5" />
                </button>
              </div>
            </div>
            
            <div className="flex-1 p-6">
              <textarea
                value={currentNote.content_md || ''}
                onChange={(e) => {
                  const updated = { ...currentNote, content_md: e.target.value };
                  setCurrentNote(updated);
                }}
                onBlur={() => updateNote({ content_md: currentNote.content_md })}
                placeholder="Start writing..."
                className="w-full h-full bg-transparent border-none outline-none resize-none text-lg"
              />
            </div>
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-gray-400">
            <div className="text-center">
              <p className="text-xl mb-2">No note selected</p>
              <p className="text-sm">Create a new note or select one from the sidebar</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
