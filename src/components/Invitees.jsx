import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { confirm, message } from '@tauri-apps/plugin-dialog';

const Invitees = () => {
  const [invitees, setInvites] = useState([]); // State to store invitees
  const [selectedIds, setSelectedIds] = useState([]); // State to store selected invite IDs
  const [showModal, setShowModal] = useState(false); // State to control modal visibility
  const [formData, setFormData] = useState({ id: null, firstname: '', lastname: '', email: '' }); // State for form data
  const [isEditMode, setIsEditMode] = useState(false); // State to track if modal is in edit mode

  useEffect(() => {
    fetchInvites();
  }, []);

  useEffect(() => {
  }, [invitees]);

  const fetchInvites = async () => {
    try {
      const response = await invoke("get_all_invite"); // Fetch invitees from Tauri
      setInvites(response); // Update state with the response
    } catch (error) {
      console.error('Error fetching invitees:', error);
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

  const handleDeleteSelected = async () => {
    const confirmation = await confirm('Voulez-vous vraiment supprimer les invites sélectionnés ?', {
          title: 'Logout',
          type: 'warning',
        });
    if(confirmation){
      try {
        await invoke("delete_invite", { ids: selectedIds });
        fetchInvites();
      } catch (error) {
        console.error('Error deleting invitees:', error);
      }
    }
  };

  // Handle opening the modal for adding a new invite
  const handleAdd = () => {
    setFormData({id: null, firstname: '', lastname: '', email: '' }); // Reset form data
    setIsEditMode(false); // Set to add mode
    setShowModal(true); // Show the modal
  };

  // Handle opening the modal for updating an invite
  const handleEdit = (invite) => {
    setFormData(invite); // Set form data to the selected invite
    setIsEditMode(true); // Set to edit mode
    setShowModal(true); // Show the modal
  };

  // Handle form submission (add or update)
  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      if (isEditMode) {
        // Update existing invite
        const response = await invoke("update_invite", {id: formData.id, invite: {firstname: formData.firstname, lastname: formData.lastname, email: formData.email}});
        if(response == "Email already exists for another invite"){
          await message("Un invite avec cet email existe déjà !", { title: "Erreur", type: "error" });
          return;
        }
      } else {
        // Add new invite
        await invoke("create_invite", {invite: {firstname: formData.firstname, lastname: formData.lastname, email: formData.email}});
      }
      setShowModal(false); // Close the modal
      fetchInvites(); // Refresh the invitees list
    } catch (error) {
      if (error == "BACKEND: Failed to create invite: UNIQUE constraint failed: invite.email") {
        await message("Un invite avec cet email existe déjà !", { title: "Erreur", type: "error" });
        return;
      }
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
        <h1 className="col">Liste des Invites</h1>
        <div className="col text-end">
          <button className="btn btn-primary me-2" onClick={handleAdd}>
          <i className="bi bi-person-fill-add fs-5"></i> &nbsp; Ajouter
          </button>
          <button
            className="btn btn-danger"
            onClick={handleDeleteSelected}
            disabled={selectedIds.length === 0}
          >
            <i className="bi bi-person-fill-dash fs-5"></i> &nbsp; Supprimer
          </button>
        </div>
      </div>
      <table className="table table-hover custom-rounded-table">
        <thead>
          <tr>
            <th scope="col">
              <input
                type="checkbox"
                style={{ transform: 'scale(1.75)', margin: '5px' }}                
                checked={selectedIds.length === invitees.length && invitees.length > 0}
                onChange={(e) => {
                  if (e.target.checked) {
                    setSelectedIds(invitees.map((invite) => invite.id));
                  } else {
                    setSelectedIds([]);
                  }
                }}
              />
            </th>
            <th scope="col">ID</th>
            <th scope="col">Nom</th>
            <th scope="col">Prenom</th>
            <th scope="col">Email</th>
            <th scope="col">Modifier</th>
          </tr>
        </thead>
        <tbody className="table-group-divider">
          {invitees.map((invite) => (
            <tr key={invite.id}>
              <td>
                <input
                  type="checkbox"
                  style={{ transform: 'scale(1.75)', margin: '5px' }}
                  checked={selectedIds.includes(invite.id)}
                  onChange={() => handleCheckboxChange(invite.id)}
                />
              </td>
              <th>{invite.id}</th>
              <td>{invite.firstname}</td>
              <td>{invite.lastname}</td>
              <td>{invite.email}</td>
              <td>
                <button className="btn btn-primary" onClick={() => handleEdit(invite)}>
                  {/* &nbsp;<i className="bi bi-pencil-square"></i>&nbsp; */}
                  &nbsp;<i className="bi bi-person-fill-gear fs-5"></i>&nbsp;
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {/* Add/Update Modal */}
      {showModal && (
        <div className="modal fade show" style={{ display: 'block', backgroundColor: 'rgba(0,0,0,0.5)' }}>
          <div className="modal-dialog">
            <div className="modal-content">
              <div className="modal-header">
                <h5 className="modal-title">{isEditMode ? 'Modifier Invite' : 'Ajouter Invite'}</h5>
                <button type="button" className="btn-close" onClick={() => setShowModal(false)}></button>
              </div>
              <div className="modal-body">
                <form onSubmit={handleSubmit}>
                  <div className="mb-3">
                    <label htmlFor="firstname" className="form-label">Nom</label>
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
                    <label htmlFor="lastname" className="form-label">Prenom</label>
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
                    <label htmlFor="email" className="form-label">Email</label>
                    <input
                      type="email"
                      className="form-control"
                      id="email"
                      name="email"
                      value={formData.email}
                      onChange={handleInputChange}
                      required
                    />
                  </div>
                  <button type="submit" className="btn btn-primary">
                    {isEditMode ? 'Modifier' : 'Ajouter'}
                  </button>
                  <button type="button" className="btn btn-danger ms-2" onClick={() => setShowModal(false)}>
                    Annuler
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

export default Invitees;