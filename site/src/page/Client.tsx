import { useClient } from 'hooks/useClient';
import { useParams } from 'react-router';
import AutoUpdateInput from 'components/AutoUpdateInput';
import { useCallback } from 'react';
import Breadcrumb from 'components/Breadcrumb';
import { BorderedTableCell } from 'components/BorderedTableCell';

export const Client = () => {
  const { clientUuid } = useParams();
  const { client, updateClient } = useClient(clientUuid);

  const handleFieldChange = useCallback(
    (field: string, value: string) => {
      if (!client || value === client[field]) {
        return; // No change, no need to update
      }
      const newClient = {
        ...client,
        [field]: value,
      };

      delete newClient.updatedAt;
      delete newClient.createdAt;

      updateClient(newClient);
    },
    [client, updateClient],
  );

  if (!client) {
    return null;
  }

  return (
    <div className="flex flex-col">
      <Breadcrumb client={client} />
      <table>
        <tbody>
          <tr>
            <BorderedTableCell>Client Name</BorderedTableCell>
            <BorderedTableCell>
              <AutoUpdateInput
                type="text"
                placeholder="Client name"
                className="text-4xl"
                serverValue={client.name}
                onChange={(value) => handleFieldChange('name', value)}
              />
            </BorderedTableCell>
          </tr>
          <tr>
            <BorderedTableCell>Description</BorderedTableCell>
            <BorderedTableCell>
              <AutoUpdateInput
                type="textarea"
                serverValue={client.description}
                onChange={(value) => handleFieldChange('description', value)}
              />
            </BorderedTableCell>
          </tr>
          <tr>
            <BorderedTableCell>Created</BorderedTableCell>
            <BorderedTableCell>{client.createdAt}</BorderedTableCell>
          </tr>
          <tr>
            <BorderedTableCell>Updated</BorderedTableCell>
            <BorderedTableCell>{client.updatedAt}</BorderedTableCell>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default Client;
