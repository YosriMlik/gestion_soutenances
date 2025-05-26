import React, { useState, useEffect } from "react";
import { Link, useParams } from "react-router-dom";
import { invoke } from '@tauri-apps/api/core';
import { confirm, message } from '@tauri-apps/plugin-dialog';
import Select from 'react-select';

const Defences = () => {
  const { id } = useParams(); // Get the department ID from the URL
  const [department, setDepartment] = useState({ name: '' });

  const [defences, setDefences] = useState([]); // All defences
  const [filteredDefences, setFilteredDefences] = useState([]); // Filtered defences
  const [classrooms, setClassrooms] = useState([]);
  const [students, setStudents] = useState([]);
  const [juries, setJuries] = useState([]);
  const [invitees, setInvitees] = useState([]);

  const [showModal, setShowModal] = useState(false);
  const [selectedDate, setSelectedDate] = useState('');
  const [selectedClassroom, setSelectedClassroom] = useState('');
  const [selectedJuries, setSelectedJuries] = useState([]);
  const [selectedInvitees, setSelectedInvitees] = useState([]);
  const [selectedStudents, setSelectedStudents] = useState([]);
  const [date, setDate] = useState('');
  const [hour, setHour] = useState('');
  const [projectName, setProjectName] = useState('');

  const [msg, setMsg] = useState('');
  const [selectedDefences, setSelectedDefences] = useState([]); // Track selected defences

  // Fetch data on component mount
  useEffect(() => {
    fetchDepartmentDetails();
    fetchDefences();
    fetchClassrooms();
    fetchJuries();
    fetchInvitees();
    fetchStudents();
  }, [id, showModal]);

  // Fetch students without a defence when the modal is shown
  const fetchStudents = async () => {
    try {
      // Get all students for the specialite
      const response = await invoke('get_specialite_students', { specialiteId: parseInt(id) });
      setStudents(response);
    } catch (error) {
      console.error('Error fetching students:', error);
    }
  };

  const fetchDepartmentDetails = async () => {
    try {
      const response = await invoke('get_specialite', { id: parseInt(id) });
      setDepartment(response);
    } catch (error) {
      console.error('Error fetching department details:', error);
    }
  };

  const fetchDefences = async () => {
    try {
      const response = await invoke('get_specialite_soutenances', { specialiteId: parseInt(id) });
      setDefences(response);
      setFilteredDefences(response); // Initialize filtered defences with all defences
    } catch (error) {
      console.error('Error fetching defences:', error);
    }
  };

  const fetchClassrooms = async () => {
    try {
      const response = await invoke('get_all_classrooms');
      setClassrooms(response);
    } catch (error) {
      console.error('Error fetching classrooms:', error);
    }
  };

  const fetchJuries = async () => {
    try {
      const response = await invoke('get_all_jury');
      setJuries(response);
    } catch (error) {
      console.error('Error fetching juries:', error);
    }
  };

  const fetchInvitees = async () => {
    try {
      const response = await invoke('get_all_invite');
      setInvitees(response);
    } catch (error) {
      console.error('Error fetching invitees:', error);
    }
  };

  // Handle filtering defences on the frontend
  const handleFilterSubmit = (e) => {
    e.preventDefault();

    const filtered = defences.filter((defence) => {
      const matchesDate = selectedDate ? defence.date?.substring(0, 10) === selectedDate : true;
      // Ensure selectedClassroom is an ID for comparison, or defence.classroom exists
      const matchesClassroom = selectedClassroom ? (defence.classroom && defence.classroom.id === parseInt(selectedClassroom)) : true;
      return matchesDate && matchesClassroom;
    });

    setFilteredDefences(filtered);
  };

  const handleDelete = async () => {
    const confirmation = await confirm("Supprimer tous ces soutenances ?", {
      title: 'Delete',
      type: 'warning',
    });

    if (confirmation) {
      try {
        if (selectedDefences.length === 0) {
          alert('Veuillez sélectionner au moins une défense à supprimer.');
          return;
        }

        // Delete each selected defence
        for (const defence of selectedDefences) {
          await invoke('delete_soutenance', { id: defence.id });
        }

        setMsg('Defences deleted successfully.');
        setSelectedDefences([]); // Clear the selected defences
        fetchDefences(); // Refresh the list
      } catch (error) {
        console.error('Error deleting defences:', error);
        setMsg('Error deleting defences.');
      }
    }
  };

  const handleAddDefenceSubmit = async (e) => {
    e.preventDefault();
    try {
      const soutenanceData = {
        specialiteId: parseInt(id),
        // selectedClassroom is an object {id, name} from the modal's state
        classroomId: selectedClassroom && selectedClassroom.id ? parseInt(selectedClassroom.id) : null,
        date,
        hour,
        pfe: projectName
      };
  
      // newSoutenance will now have the full structure (classroom populated, others empty arrays)
      const newSoutenance = await invoke('create_soutenance', soutenanceData);
      // Update students with the new soutenance_id
      for (const student of selectedStudents) {
        console.log("new soutenance", newSoutenance.id);
        const res = await invoke('update_student', {
          id: student.id,
          firstname: student.firstname,
          lastname: student.lastname,
          address: student.address,
          specialiteId: student.specialite_id,
          soutenanceId: newSoutenance.id
        });
        console.log("res", res);  
      }

      // Create jury-soutenance relationships
      for (const jury of selectedJuries) {
        await invoke('create_jury_soutenance', {
          juryId: jury.id,
          soutenanceId: newSoutenance.id,
          role: 'member' // You might want to make this configurable
        });
      }

      // Create invite-soutenance relationships
      for (const invitee of selectedInvitees) {
        await invoke('create_invite_soutenance', {
          inviteId: invitee.id,
          soutenanceId: newSoutenance.id,
        });
      }

      setProjectName("");
      setDate("");
      setHour("");
      setSelectedStudents([]);
      setSelectedJuries([]);
      setSelectedInvitees([]);
      setSelectedClassroom('');
      setShowModal(false); // Close the modal
      fetchDefences(); // Refresh the list
    } catch (error) {
      console.error('Error creating defence:', error);
    }
  };

  return (
    <div className="mx-auto mt-5 _container">
      {/* Add Modal */}
      {showModal && (
        <div className="modal fade show" style={{ display: 'block', backgroundColor: 'rgba(0,0,0,0.5)' }}>
          <div className="modal-dialog">
            <div className="modal-content">
              <div className="modal-header">
                <h5 className="modal-title"><b>Ajouter une Soutenance</b></h5>
                <button type="button" className="btn-close" onClick={() => setShowModal(false)}></button>
              </div>
              <div className="modal-body">
                <form onSubmit={handleAddDefenceSubmit}>
                  <div className="mb-3">
                    <label className="form-label">Salle</label>
                    <select
                      className="form-select"
                      name="salle"
                      value={selectedClassroom.id || ''}
                      onChange={(e) => setSelectedClassroom({ id: e.target.value, name: classrooms.find(classroom => classroom.id === parseInt(e.target.value))?.name })}
                    >
                      <option value="">Sélectionner Salle</option>
                      {classrooms.map((salle) => (
                        <option key={salle.id} value={salle.id}>
                          {salle.name}
                        </option>
                      ))}
                    </select>
                  </div>
                  <div className="row mb-3">
                    <div className="col-8">
                      <label htmlFor="date" className="form-label">Date</label>
                      <input
                        type="date"
                        className="form-control"
                        id="date"
                        value={date}
                        onChange={(e) => setDate(e.target.value)}
                        required
                      />
                    </div>
                    <div className="col-4">
                      <label htmlFor="hour" className="form-label">Heure</label>
                      <input
                        type="time"
                        className="form-control"
                        id="hour"
                        value={hour}
                        onChange={(e) => setHour(e.target.value)}
                      />
                    </div>
                  </div>
                  <div className="mb-3">
                    <label className="form-label">Étudiants</label>
                    <Select
                      isMulti
                      closeMenuOnSelect={false}
                      value={selectedStudents.map(student => ({
                        value: student.id,
                        label: `${student.firstname} ${student.lastname}`
                      }))}
                      onChange={(selected) => {
                        setSelectedStudents(students.filter(student => selected.map(s => s.value).includes(student.id)));
                      }}
                      options={students.map((item) => ({
                        value: item.id,
                        label: `${item.firstname} ${item.lastname}`,
                      }))}
                    />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="projectName" className="form-label">Nom du Projet</label>
                    <input
                      type="text"
                      className="form-control"
                      id="projectName"
                      value={projectName}
                      onChange={(e) => setProjectName(e.target.value)}
                    />
                  </div>
                  <div className="mb-3">
                    <label className="form-label">Jurys</label>
                    <Select
                      isMulti
                      closeMenuOnSelect={false}
                      value={selectedJuries.map(jury => ({
                        value: jury.id,
                        label: `${jury.firstname} ${jury.lastname}`
                      }))}
                      onChange={(selected) => {
                        setSelectedJuries(juries.filter(jury => selected.map(s => s.value).includes(jury.id)));
                      }}
                      options={juries.map((item) => ({
                        value: item.id,
                        label: `${item.firstname} ${item.lastname}`,
                      }))}
                    />
                  </div>
                  <div className="mb-3">
                    <label className="form-label">Invitées</label>
                    <Select
                      isMulti
                      closeMenuOnSelect={false}
                      value={selectedInvitees.map(invitee => ({
                        value: invitee.id,
                        label: `${invitee.firstname} ${invitee.lastname}`
                      }))}
                      onChange={(selected) => {
                        setSelectedInvitees(invitees.filter(invitee => selected.map(s => s.value).includes(invitee.id)));
                      }}
                      options={invitees.map((item) => ({
                        value: item.id,
                        label: `${item.firstname} ${item.lastname}`,
                      }))}
                    />
                  </div>
                  <button type="submit" className="btn btn-primary">Ajouter</button>
                </form>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Header */}
      <div className="row g-0">
        {msg && <span hidden id="sent">{msg}</span>}
        <h1 className="col-lg-10 col-md-8 col-6">
          Spécialité: {department.name}
        </h1>
        <div className="col-lg-2 col-md-4 col-6 text-end dropdown">
          <Link
            className="btn btn-primary ms-5 px-3"
            style={{ borderRadius: 0 }}
            to="#Gérer"
          >
            <i className="bi bi-sliders2-vertical"></i>
            <span className="ms-1">Gérer</span>
          </Link>
          <div className="dropdown-content" style={{ right: 0 }}>
            <Link to={`/dashboard/department/${id}/defences`}>Soutenances</Link>
            <Link to={`/dashboard/department/${id}/students`}>Etudiants</Link>
          </div>
        </div>
      </div>

      {/* Filter Form */}
      <br />
      <div className="row g-0">
        <div className="col-10">
          <form className="row ps-0" method="GET" onSubmit={handleFilterSubmit}>
            <div className="col-auto pt-1">Filtrer par :</div>
            <div className="col-auto">
              <input
                className="form-control"
                type="date"
                name="date"
                value={selectedDate}
                onChange={(e) => setSelectedDate(e.target.value)}
              />
            </div>
            <div className="col-auto">
              <select
                className="form-select"
                name="salle"
                value={selectedClassroom}
                onChange={(e) => setSelectedClassroom(e.target.value)}
              >
                <option value="">Sélectionner Salle</option>
                {classrooms.map((salle) => (
                  <option key={salle.id} value={salle.id}>
                    {salle.name}
                  </option>
                ))}
              </select>
            </div>
            <div className="col-auto">
              <button type="submit" className="btn rounded-1 btn-primary ms-3 py-1">
                <i className="bi bi-search"></i>
              </button>
            </div>
            <div className="col-auto">
              {/* <button
                type="button"
                className="btn rounded-1 btn-primary py-1"
                onClick={() => setShowModal(true)}
              >
                <i className="bi bi-folder-plus"></i>
              </button> */}
            </div>
          </form>
        </div>
        <div className="col-2" style={{ textAlign: 'right' }}>
          <button
            type="button"
            className="btn rounded-1 btn-primary pt-2 me-3"
            onClick={() => setShowModal(true)}
          >
            <i className="bi bi-folder-plus"></i>
          </button>
          <button
            className="btn rounded-1 btn-danger pt-2"
            onClick={handleDelete}
            disabled={selectedDefences.length === 0} // Disable if no defences are selected
          >
            <i className="bi bi-trash3"></i>
          </button>
        </div>
      </div>

      {/* Table */}
      <br />
      <br />
      <table className="table table-hover">
        <thead>
          <tr>
            <th scope="col">-</th>
            <th scope="col">Elève(s)</th>
            <th scope="col">PFE</th>
            <th scope="col">Jury</th>
            <th scope="col">Invités</th>
            <th scope="col">Date</th>
            <th scope="col">Heure</th>
            <th scope="col">Salle</th>
          </tr>
        </thead>
        <tbody className="table-group-divider">
          {filteredDefences.length > 0 ? (
            filteredDefences.map((defence) => (
              <tr
                key={defence.id}
                id={defence.id}
                className={selectedDefences.includes(defence.id) ? 'table-active' : ''} // Highlight selected rows
              >
                <td>
                  <input
                    className="form-check-input"
                    type="checkbox"
                    id={`rowN-${defence.id}`}
                    onChange={(e) => {
                      const defenceId = defence.id;
                      if (e.target.checked) {
                        // Add the defence ID as an object
                        setSelectedDefences((prev) => [...prev, { id: defenceId }]);
                      } else {
                        // Remove the defence ID object
                        setSelectedDefences((prev) => prev.filter((item) => item.id !== defenceId));
                      }
                    }}
                    checked={selectedDefences.some((item) => item.id === defence.id)} // Check if the defence is selected
                  />
                </td>
                <td>
                  {defence.students?.map((student) => (
                    <div key={student.id}>
                      {student.firstname} <span style={{ textTransform: 'uppercase' }}>{student.lastname}</span>
                    </div>
                  ))}
                </td>
                <td>{defence.pfe}</td>
                <td>
                  {defence.juries?.map((jury) => (
                    <div key={jury.id}>
                      {jury.firstname} <span style={{ textTransform: 'uppercase' }}>{jury.lastname}</span>
                    </div>
                  ))}
                </td>
                <td>
                  {defence.invitees?.map((invite) => (
                    <div key={invite.id}>
                      {invite.firstname} <span style={{ textTransform: 'uppercase' }}>{invite.lastname}</span>
                    </div>
                  ))}
                </td>
                <td>{defence.date?.substring(0, 10)}</td>
                <td>{defence.hour}</td>
                <td>{defence.classroom?.name}</td>
              </tr>
            ))
          ) : (
            // Optional: Render a placeholder row or message if there are no defences
            <tr>
              <td colSpan="8" className="text-center">Pas de soutenances pour le moment</td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
};

export default Defences;
