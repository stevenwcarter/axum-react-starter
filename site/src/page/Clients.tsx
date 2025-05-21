import { useClients } from 'hooks/useClients';
import { useState } from 'react';
import Button from '../components/Button';
import { Link } from 'react-router';
import { Client } from 'types';

export const Clients = () => {
  const { clients, createClient, deleteClient } = useClients();

  const [name, setName] = useState<string>('');

  const handleNameUpdate = (event: React.ChangeEvent<HTMLInputElement>) => {
    setName(event.target.value);
  };
  const handleNameKeyUp = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      createClient({ name, description: '', archived: false });
      setName('');
    }
  };

  const handleDelete = (clientUuid: string) => {
    deleteClient(clientUuid);
  };

  return (
    <div>
      <h1>Clients</h1>
      <div className="flex flex-col gap-2 mt-4 border rounded-lg p-4 bg-slate-500/10 w-96 border-slate-400">
        <p>Add a new client</p>
        <input
          type="text"
          value={name}
          placeholder="Client name"
          onChange={handleNameUpdate}
          onKeyUp={handleNameKeyUp}
        />
      </div>
      <div className="flex flex-wrap mt-3 gap-2">
        {clients.map((client) => (
          <ClientItem key={client.uuid} client={client} handleDelete={handleDelete} />
        ))}
      </div>
    </div>
  );
};

interface ClientItemProps {
  client: Client;
  handleDelete: (clientUuid: string) => void;
}

const ClientItem = (props: ClientItemProps) => {
  const { client, handleDelete } = props;

  return (
    <Link to={`/client/${client.uuid}`}>
      <div
        className="flex p-4 gap-2 flex-col bg-slate-600 w-96 rounded-lg hover:bg-slate-500"
        key={client.uuid}
      >
        <h3>{client.name}</h3>
        <div className="font-thin text-sm">{client.description}</div>
        <Button className="hidden" size={'sm'} onClick={() => handleDelete(client.uuid)}>
          Delete
        </Button>
      </div>
    </Link>
  );
};

export default Clients;
