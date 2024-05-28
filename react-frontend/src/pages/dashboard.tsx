import { useEffect, useState } from 'react';
import {
    create,
    type CredentialCreationOptionsJSON,
} from "@github/webauthn-json";
import { useNavigate } from "react-router-dom";


import { Button } from "@/components/ui/button";
import { toast } from "sonner"

interface Credential {
    id: string;
    name: string;
    last_used_at: string;
    created_at: string;
}


const Dashboard = () => {
    const navigate = useNavigate();
    const [credentials, setCredentials] = useState<Credential[]>([]);
    const [editName, setEditName] = useState<{ [key: string]: string }>({});

    useEffect(() => {
        fetchCredentials();
    }, []);

    const fetchCredentials = async () => {
        const response = await fetch('http://localhost:8080/passkeys/credentials', {
            method: 'GET',
            credentials: 'include',
        });
        const data = await response.json();
        if (response.ok) {
            setCredentials(data.credentials);
        } else {
            console.error('Failed to fetch credentials');
        }
        fetchCredentials();
    };

    const handleLogout = async () => {
        try {
            const response = await fetch('http://localhost:8080/logout', {
                method: 'POST',
                credentials: 'include',
            });
            const data = await response.json();
            if (response.ok) {
                console.log('Logout successful:', data);
                navigate('/login');
            } else {
                console.error('Logout failed:', data.message);
            }
        } catch (error) {
            console.error('Logout error:', error);
        }
    };

    async function registerPasskey() {
        const createOptionsResponse = await fetch("http://localhost:8080/passkeys/start-registration", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            credentials: 'include',
            body: JSON.stringify({ start: true, finish: false, credential: null }),
        });

        const { createOptions } = await createOptionsResponse.json();
        console.log("createOptions", createOptions)

        const credential = await create(
            createOptions as CredentialCreationOptionsJSON,
        );
        console.log(credential)

        const response = await fetch("http://localhost:8080/passkeys/finalize-registration", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            body: JSON.stringify({ start: false, finish: true, credential }),
        });
        console.log(response)

        if (response.ok) {
            toast.success("Registered passkey successfully!");
            return;
        }
    }


    const updateCredential = async (credentialID: string, updatedData: { name: string }) => {
        try {
            const response = await fetch(`http://localhost:5001/api/passkeys/credentials/${credentialID}`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    credentials: 'include',
                },
                body: JSON.stringify(updatedData),
            });
    
            if (!response.ok) {
                throw new Error('Failed to update credential');
            }
    
            const responseData = await response.json();
            console.log('Update successful:', responseData);
    
            // Update the local state to reflect the changed name
            setCredentials(currentCredentials =>
                currentCredentials.map(cred =>
                    cred.id === credentialID ? { ...cred, name: updatedData.name } : cred
                )
            );
    
            // Show a success message
            toast.success("Credential updated successfully!");
        } catch (error) {
            console.error('Error updating credential:', error);
            // Show an error message to the user
            toast.error("Failed to update credential.");
        }
    };


    const deleteCredential = async (credentialID: string) => {
        try {
            const response = await fetch(`http://localhost:5001/api/passkeys/credentials/${credentialID}`, {
                method: 'DELETE',
                credentials: 'include',
            });
            if (!response.ok) {
                throw new Error('Failed to delete the credential');
            }
            // Remove the deleted credential from the state to update the UI
            setCredentials(credentials.filter(cred => cred.id !== credentialID));
            console.log("Deleted credential successfully");
        } catch (error) {
            console.error('Error deleting credential:', error);
            // Optionally, show an error message to the user
        }
    };
    return (
        <div className="p-4">
            <h1>Dashboard</h1>
            <Button onClick={handleLogout}>Logout</Button>
            <div>
                <Button
                    onClick={() => registerPasskey()}
                    className="flex justify-center items-center space-x-2 mt-8"
                >
                    Register a new passkey
                </Button>
            </div>
            <div className="mt-8">
                <h2>My Credentials</h2>
                {credentials.length > 0 ? (
                    credentials.map((cred) => (
                        <div key={cred.id} className="p-4 border rounded mb-2">
                            <p>Id: {cred.id}</p>
                            <p>Name: {cred.name}</p>
                            <input
                                type="text"
                                value={editName[cred.id] || ''}
                                onChange={(e) => setEditName({ ...editName, [cred.id]: e.target.value })}
                                placeholder="New name"
                            />
                            <p>Last Used At: {new Date(cred.last_used_at).toLocaleString()}</p>
                            <p>Created At: {new Date(cred.created_at).toLocaleString()}</p>
                            <Button onClick={() => deleteCredential(cred.id)}>Delete</Button>
                            <Button onClick={() => updateCredential(cred.id, { name: editName[cred.id] })}>
                                Update Name
                            </Button>
                        </div>
                    ))
                ) : (
                    <p>No passkeys registered yet. Click the button above to register a new passkey.</p>
                )}
            </div>
        </div>
    )
}

export default Dashboard;