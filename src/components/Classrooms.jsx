import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

const Classrooms = () => {
  const [classrooms, setClassrooms] = useState([]); // State to store classrooms
  const [selectedIds, setSelectedIds] = useState([]); // State to store selected classroom IDs
  const [showModal, setShowModal] = useState(false); // State to control modal visibility
  const [formData, setFormData] = useState({ id: null, name: ''}); // State for form data
  const [isEditMode, setIsEditMode] = useState(false); // State to track if modal is in edit mode

  // Fetch classrooms from the Tauri backend
  useEffect(() => {
    fetchClassrooms();
  }, []);

  const fetchClassrooms = async () => {
    try {
      const response = await invoke('get_all_classrooms');
      setClassrooms(response);
    } catch (error) {
      console.error('Error fetching classrooms:', error);
    }
  };

  // Handle checkbox selection
  const handleCheckboxChange = (id) => {
    if (selectedIds.includes(id)) {
      setSelectedIds(selectedIds.filter((selectedId) => selectedId !== id));
    } else {
      setSelectedIds([...selectedIds, id]);
    }
  };

  // Handle bulk deletion
  const handleDeleteSelected = async () => {
    const confirmation = await confirm('Voulez-vous vraiment supprimer les salles sélectionnés ?', {
      title: 'Logout',
      type: 'warning',
    });
    if(confirmation){
      try {
        await invoke("delete_classrooms", { ids: selectedIds });
        setSelectedIds([]);
        fetchClassrooms();
      } catch (error) {
        console.error('Error deleting classroomss:', error);
      }
    }
  };

  // Handle opening the modal for adding a new classroom
  const handleAdd = () => {
    setFormData({ id: null, name: ''}); // Reset form data
    setIsEditMode(false); // Set to add mode
    setShowModal(true); // Show the modal
  };

  // Handle opening the modal for updating a classroom
  const handleEdit = (classroom) => {
    setFormData(classroom); // Set form data to the selected classroom
    setIsEditMode(true); // Set to edit mode
    setShowModal(true); // Show the modal
  };

  // Handle form submission (add or update)
  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      if (isEditMode) {
        // Update existing classroom
        await invoke('update_classroom', { 
          id: formData.id,
          name: formData.name 
        });
      } else {
        // Add new classroom
        await invoke('create_classroom', { name: formData.name });
      }
      setShowModal(false); // Close the modal
      fetchClassrooms(); // Refresh the classrooms list
    } catch (error) {
      console.error('Error saving classroom:', error);
    }
  };

  // Handle form input changes
  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData({ ...formData, [name]: value });
  };

  return (
    <div className="mx-auto mt-5">
      <div className="row g-0">
        <h1 className="col">Liste des Salles</h1>
        <div className="col text-end">
          <button className="btn btn-primary me-2" onClick={handleAdd}>
            + Ajouter
          </button>
          <button
            className="btn btn-danger"
            onClick={handleDeleteSelected}
            disabled={selectedIds.length === 0}
          >
            Supprimer
          </button>
        </div>
      </div>
      <table className="table table-hover">
        <thead>
          <tr>
            <th scope="col">
              <input
                type="checkbox"
                style={{ transform: 'scale(1.75)', margin: '5px' }}
                checked={selectedIds.length === classrooms.length && classrooms.length > 0}
                onChange={(e) => {
                  if (e.target.checked) {
                    setSelectedIds(classrooms.map((classroom) => classroom.id));
                  } else {
                    setSelectedIds([]);
                  }
                }}
              />
            </th>
            <th scope="col">ID</th>
            <th scope="col">Nom</th>
            <th scope="col">Action</th>
          </tr>
        </thead>
        <tbody className="table-group-divider">
          {classrooms.map((classroom) => (
            <tr key={classroom.id}>
              <td>
                <input
                  type="checkbox"
                  style={{ transform: 'scale(1.75)', margin: '5px' }}
                  checked={selectedIds.includes(classroom.id)}
                  onChange={() => handleCheckboxChange(classroom.id)}
                />
              </td>
              <th>{classroom.id}</th>
              <td>{classroom.name}</td>
              <td>
                <button className="btn btn-primary" onClick={() => handleEdit(classroom)}>
                  Modifier
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {/* Add/Update Modal */}
      {showModal && (
        <div className="modal fade show" style={{ display: 'block', backgroundColor: 'rgba(0,0,0,0.5)' }}>
          <div className="modal-dialog" style={{ marginTop: "150px" }}>
            <div className="modal-content">
              <div className="modal-header">
                <h5 className="modal-title">{isEditMode ? 'Modifier Salle' : 'Ajouter Salle'}</h5>
                <button type="button" className="btn-close" onClick={() => setShowModal(false)}></button>
              </div>
              <div className="modal-body">
                <form onSubmit={handleSubmit}>
                  <div className="mb-3">
                    <label htmlFor="name" className="form-label">Nom</label>
                    <input
                      type="text"
                      className="form-control"
                      id="name"
                      name="name"
                      value={formData.name}
                      onChange={handleInputChange}
                      required
                    />
                  </div>
                  <button type="submit" className="btn btn-primary">
                    {isEditMode ? 'Modifier' : 'Ajouter'}
                  </button>
                </form>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Classrooms;