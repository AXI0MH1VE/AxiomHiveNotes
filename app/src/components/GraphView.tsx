import { useEffect, useState, useRef } from 'react';
import ForceGraph2D from 'react-force-graph-2d';
import { api } from '../lib/api';
import { X } from 'lucide-react';

interface GraphViewProps {
  onClose: () => void;
  onNavigate: (noteId: number) => void;
}

interface GraphData {
  nodes: Array<{ id: number; title: string }>;
  links: Array<{ source: number; target: number }>;
}

export function GraphView({ onClose, onNavigate }: GraphViewProps) {
  const [graphData, setGraphData] = useState<GraphData | null>(null);
  const [loading, setLoading] = useState(true);
  const graphRef = useRef<any>(null);

  useEffect(() => {
    loadGraphData();
  }, []);

  const loadGraphData = async () => {
    setLoading(true);
    try {
      const data = await api.getGraphData();
      setGraphData(data);
    } catch (error) {
      console.error('Failed to load graph data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleNodeClick = (node: any) => {
    onNavigate(node.id);
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-gray-900 z-50 flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-gray-700 flex items-center justify-between">
        <div>
          <h2 className="text-xl font-bold text-purple-500">Graph View</h2>
          <p className="text-sm text-gray-400">
            {graphData ? `${graphData.nodes.length} notes, ${graphData.links.length} connections` : 'Loading...'}
          </p>
        </div>
        <button
          onClick={onClose}
          className="p-2 hover:bg-gray-800 rounded transition-colors"
        >
          <X className="w-5 h-5" />
        </button>
      </div>

      {/* Graph */}
      <div className="flex-1">
        {loading ? (
          <div className="flex items-center justify-center h-full text-gray-400">
            Loading graph...
          </div>
        ) : graphData ? (
          <ForceGraph2D
            ref={graphRef}
            graphData={graphData}
            nodeLabel="title"
            nodeAutoColorBy="id"
            nodeCanvasObject={(node: any, ctx: CanvasRenderingContext2D, globalScale: number) => {
              const label = node.title;
              const fontSize = 12 / globalScale;
              ctx.font = `${fontSize}px Inter, sans-serif`;
              const textWidth = ctx.measureText(label).width;
              const bckgDimensions = [textWidth, fontSize].map(n => n + fontSize * 0.4);

              // Draw node circle
              ctx.fillStyle = '#7C3AED';
              ctx.beginPath();
              ctx.arc(node.x, node.y, 5, 0, 2 * Math.PI, false);
              ctx.fill();

              // Draw label background
              ctx.fillStyle = 'rgba(17, 24, 39, 0.9)';
              ctx.fillRect(
                node.x - bckgDimensions[0] / 2,
                node.y - bckgDimensions[1] / 2 + 8,
                bckgDimensions[0],
                bckgDimensions[1]
              );

              // Draw label text
              ctx.textAlign = 'center';
              ctx.textBaseline = 'middle';
              ctx.fillStyle = '#ffffff';
              ctx.fillText(label, node.x, node.y + 8);
            }}
            linkColor={() => '#4B5563'}
            linkWidth={1}
            onNodeClick={handleNodeClick}
            backgroundColor="#111827"
          />
        ) : (
          <div className="flex items-center justify-center h-full text-red-400">
            Failed to load graph
          </div>
        )}
      </div>
    </div>
  );
}
