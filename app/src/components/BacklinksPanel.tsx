import { useEffect, useState } from 'react';
import { api } from '../lib/api';
import { Link2 } from 'lucide-react';

interface BacklinksPanelProps {
  noteId: number;
  onNavigate: (noteId: number) => void;
}

export function BacklinksPanel({ noteId, onNavigate }: BacklinksPanelProps) {
  const [backlinks, setBacklinks] = useState<Array<[number, string]>>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadBacklinks();
  }, [noteId]);

  const loadBacklinks = async () => {
    setLoading(true);
    try {
      const links = await api.getBacklinks(noteId);
      setBacklinks(links);
    } catch (error) {
      console.error('Failed to load backlinks:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-4 border-t border-gray-700">
      <div className="flex items-center gap-2 mb-3">
        <Link2 className="w-4 h-4 text-purple-500" />
        <h3 className="font-semibold text-sm">Backlinks</h3>
        <span className="text-xs text-gray-500">({backlinks.length})</span>
      </div>
      
      {loading ? (
        <div className="text-sm text-gray-500">Loading...</div>
      ) : backlinks.length === 0 ? (
        <div className="text-sm text-gray-500">No backlinks</div>
      ) : (
        <div className="space-y-2">
          {backlinks.map(([id, title]) => (
            <button
              key={id}
              onClick={() => onNavigate(id)}
              className="w-full text-left p-2 rounded hover:bg-gray-800 transition-colors text-sm"
            >
              <div className="text-purple-400">← {title}</div>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
