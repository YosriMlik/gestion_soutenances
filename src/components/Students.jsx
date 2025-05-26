import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { confirm, message } from '@tauri-apps/plugin-dialog';
import { useParams } from 'react-router-dom';

const Students = () => {
  const { id } = useParams(); // Get the department ID from the URL
  const [ specialite, setSpecialite ] = useState(""); // Get the department ID from the URL
  const [students, setStudents] = useState([]); // State to store students
  const [selectedIds, setSelectedIds] = useState([]); // State to store selected student IDs
  const [showModal, setShowModal] = useState(false); // State to control modal visibility
  const [formData, setFormData] = useState({ 
    id: null, 
    firstname: '', 
    lastname: '', 
    address: '', 
    specialite_id: parseInt(id) 
  }); // State for form data
  const [isEditMode, setIsEditMode] = useState(false); // State to track if modal is in edit mode

  // Fetch students from the Tauri backend
  useEffect(() => {
    fetchStudents();
    getSpecialite();
  }, [id]);
  
  const getSpecialite = async () => {
    try {
      const response = await invoke('get_specialite', { id: parseInt(id) });
      setSpecialite(response.name);
    } catch (error) {
      console.error('Error fetching specialite:', error);
    }
  }

  const fetchStudents = async () => {
    try {
      const response = await invoke('get_students_by_department', { departmentId: parseInt(id) });
      setStudents(response);
    } catch (error) {
      console.error('Error fetching students:', error);
    }
  };

  // Handle checkbox selection
  const handleCheckboxChange = (studentId) => {
    if (selectedIds.includes(studentId)) {
      setSelectedIds(selectedIds.filter((selectedId) => selectedId !== studentId));
    } else {
      setSelectedIds([...selectedIds, studentId]);
    }
  };

  // Handle bulk deletion
  const handleDeleteSelected = async () => {
    const confirmation = await confirm('Êtes-vous sûr de vouloir supprimer le(s) étudiant(s) sélectionné(s) ?', {
      title: 'Logout',
      type: 'warning',
    });
    if (confirmation) {
      try {
        await invoke('delete_students', { ids: selectedIds });
        setStudents(students.filter((student) => !selectedIds.includes(student.id)));
        setSelectedIds([]);
        alert('Étudiant(s) supprimé(s) avec succès!');
      } catch (error) {
        console.error('Error deleting students:', error);
        alert('Erreur lors de la suppression des étudiants.');
      }
    }
  };

  // Handle opening the modal for adding a new student
  const handleAdd = () => {
    setFormData({ 
      id: null, 
      firstname: '', 
      lastname: '', 
      address: '', 
      specialite_id: parseInt(id) 
    }); // Reset form data
    setIsEditMode(false); // Set to add mode
    setShowModal(true); // Show the modal
  };

  // Handle opening the modal for updating a student
  const handleEdit = (student) => {
    setFormData(student); // Set form data to the selected student
    setIsEditMode(true); // Set to edit mode
    setShowModal(true); // Show the modal
  };

  // Handle form submission (add or update)
  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      if (isEditMode) {
        // Update existing student
        await invoke('update_student', {
          id: formData.id,
          firstname: formData.firstname,
          lastname: formData.lastname,
          address: formData.address
        });
      } else {
        // Add new student
        await invoke('create_student', {
          firstname: formData.firstname,
          lastname: formData.lastname,
          address: formData.address,
          specialiteId: parseInt(id)
        });
      }
      setShowModal(false); // Close the modal
      fetchStudents(); // Refresh the students list
    } catch (error) {
      console.error('Error saving student:', error);
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
        <h1 className="col">Liste des Étudiants { specialite }</h1>
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
                checked={selectedIds.length === students.length && students.length > 0}
                onChange={(e) => {
                  if (e.target.checked) {
                    setSelectedIds(students.map((student) => student.id));
                  } else {
                    setSelectedIds([]);
                  }
                }}
              />
            </th>
            <th scope="col">ID</th>
            <th scope="col">Prénom</th>
            <th scope="col">Nom</th>
            <th scope="col">Adresse</th>
            <th scope="col">Action</th>
          </tr>
        </thead>
        <tbody className="table-group-divider">
          {students.map((student) => (
            <tr key={student.id}>
              <td>
                <input
                  type="checkbox"
                  style={{ transform: 'scale(1.75)', margin: '5px' }}
                  checked={selectedIds.includes(student.id)}
                  onChange={() => handleCheckboxChange(student.id)}
                />
              </td>
              <th>{student.id}</th>
              <td>{student.firstname}</td>
              <td>{student.lastname}</td>
              <td>{student.address}</td>
              <td>
                <button className="btn btn-primary" onClick={() => handleEdit(student)}>
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
                <h5 className="modal-title">{isEditMode ? 'Modifier Étudiant' : 'Ajouter Étudiant'}</h5>
                <button type="button" className="btn-close" onClick={() => setShowModal(false)}></button>
              </div>
              <div className="modal-body">
                <form onSubmit={handleSubmit}>
                  <div className="mb-3">
                    <label htmlFor="firstname" className="form-label">Prénom</label>
                    <input
                      type="text"
                      className="form-control"
                      id="firstname"
                      name="firstname"
                      value={formData.firstname}
                      onChange={handleInputChange}
                      required
                    />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="lastname" className="form-label">Nom</label>
                    <input
                      type="text"
                      className="form-control"
                      id="lastname"
                      name="lastname"
                      value={formData.lastname}
                      onChange={handleInputChange}
                      required
                    />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="address" className="form-label">Adresse</label>
                    <input
                      type="text"
                      className="form-control"
                      id="address"
                      name="address"
                      value={formData.address}
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

export default Students;