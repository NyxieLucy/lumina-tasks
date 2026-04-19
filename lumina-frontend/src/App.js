import { formatDate, formatDistanceToNow } from 'date-fns';
import { useState, useEffect } from 'react';
import './App.css'
function App() {
  const [tasks, setTasks] = useState([]);
  const [formData, setFormData] = useState({ title: '', categorie: '', description: '' });
  const [editingId, setEditingId] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const API_URL = 'http://127.0.0.1:3001';

  // Fetch all tasks on mount
  useEffect(() => {
    fetchTasks();
  }, []);

  const fetchTasks = async () => {
    try {
      setLoading(true);
      const response = await fetch(`${API_URL}/tasks`);
      if (!response.ok) throw new Error('Failed to fetch tasks');
      const data = await response.json();
      setTasks(data);
      setError(null);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const createTask = async (e) => {
    e.preventDefault();
    if (!formData.title.trim()) {
      setError('Title is required');
      return;
    }

    try {
      const response = await fetch(`${API_URL}/tasks`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      });
      if (!response.ok) throw new Error('Failed to create task');
      
      await fetchTasks();
      setFormData({ title: '', categorie: '', description: '', progress: 0, achieved: false });
      
      setError(null);
    } catch (err) {
      setError(err.message);
    }
    
  };

  const updateTask = async (id) => {
    try {
      const response = await fetch(`${API_URL}/tasks/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          title: formData.title || undefined,
          categorie: formData.categorie || undefined,
          description: formData.description || undefined,
          progress: formData.progress !== '' ? formData.progress : undefined,
          achieved: formData.achieved !== '' ? formData.achieved : undefined,
        }),
      });
      if (!response.ok) throw new Error('Failed to update task');
      
      await fetchTasks();
      setFormData({ title: '', categorie: '', description: '', progress: 0, achieved: false });
      setEditingId(null);
      setError(null);
    } catch (err) {
      setError(err.message);
    }
  };

  const deleteTask = async (id) => {
    try {
      const response = await fetch(`${API_URL}/tasks/${id}`, {
        method: 'DELETE',
      });
      if (!response.ok) throw new Error('Failed to delete task');
      
      await fetchTasks();
      setError(null);
    } catch (err) {
      setError(err.message);
    }
  };

  const handleEdit = (task) => {
    setEditingId(task.id);
    
    setFormData({
      
      title: task.title,
      categorie: task.categorie,
      description: task.description,
      progress: task.progress || 0,
      achieved: task.achieved||false,
      
    });

  };

  const handleCancel = () => {
    setEditingId(null);
    setFormData({ title: '', categorie: '', description: '' });
  };

  return (
    <div className="App">
      <header className="header">
        <h1>✨ Lumina Tasks</h1>
        <p className="subtitle">bring light to your task management</p>
      </header>

      {error && <div className="error-banner">{error}</div>}

      <main className="container">
        {/* Form Section */}
        <section className="form-section">
          <h2>{editingId ? 'Edit Task' : 'Create New Task'}</h2>
          <form onSubmit={(e) => {
            e.preventDefault();
            editingId ? updateTask(editingId) : createTask(e);
          }}>
            <div className="form-group">
              <input
                type="text"
                placeholder="Task title"
                value={formData.title}
                onChange={(e) => setFormData({ ...formData, title: e.target.value })}
              />
            </div>
            <div className="form-group">
              <input
                type="text"
                placeholder="Category"
                value={formData.categorie}
                onChange={(e) => setFormData({ ...formData, categorie: e.target.value })}
              />
            </div>
            <div className="form-group">
              <textarea
                placeholder="Description"
                value={formData.description}
                onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              />
            </div>
            
            {editingId && (
              <>
                <div className="form-group">
                  <label>Progress: {formData.progress}%</label>
                  <input
                    type="range"
                    min="0"
                    max="100"
                    value={formData.progress || 0}
                    onChange={(e) => setFormData({ ...formData, progress: parseInt(e.target.value) })}
                  />
                </div>
                <div className="form-group">
                  <label>
                    <input
                      type="checkbox"
                      checked={formData.achieved || false}
                      onChange={(e) => setFormData({ ...formData, achieved: e.target.checked })}
                    />
                    {' '}Mark as achieved
                  </label>
                </div>
              </>
            )}
            
            <div className="form-actions">
              <button type="submit" className="btn-primary">
                {editingId ? 'Update Task' : 'Create Task'}
              </button>
              {editingId && (
                <button type="button" className="btn-secondary" onClick={handleCancel}>
                  Cancel
                </button>
              )}
            </div>
          </form>
        </section>

        {/* Tasks List Section */}
        <section className="tasks-section">
          <h2>Your Tasks</h2>
          {loading && <p className="loading">Loading tasks...</p>}
          {!loading && tasks.length === 0 && <p className="empty">No tasks yet. Create one to get started!</p>}
          
          <div className="tasks-grid">
            {tasks.map((task) => (
              <div key={task.id} className="task-card">
                   <div className="task-timestamps">
                     <small>Created: {formatDistanceToNow(new Date(task.created_at), { addSuffix: true })}</small> <br></br>
                     <small>Updated: {formatDistanceToNow(new Date(task.updated_at), { addSuffix: true })}</small>
                   </div>
                   <div className="task-header">
                     <h3>{task.title}</h3>
                     <span className="category-badge">{task.categorie || 'uncategorized'}</span>
                   </div>
                <p className="task-description">{task.description}</p>
                <div className="form-group">
                  <label>Progress: {formData.progress}%</label>
                  <input
                    type="range"
                    min="0"
                    max="100"
                    value={formData.progress}
                    onChange={(e) => setFormData({ ...formData, progress: parseInt(e.target.value) })}
                  />
                </div>
                <div className="task-footer">
                  <span className={`progress-badge ${task.achieved ? 'achieved' : ''}`}>
                    {task.achieved ? '✓ Done' : `${task.progress}%`}
                  </span>
                  
                  <div className="task-actions">
                    <button className="btn-edit" onClick={() => handleEdit(task)}>Edit</button>
                    <button className="btn-delete" onClick={() => deleteTask(task.id)}>Delete</button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;