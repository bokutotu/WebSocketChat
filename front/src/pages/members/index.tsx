import { useEffect, useState } from 'react';

import { getMembers, postMember } from '../../features/api';

import { Member } from '../../types/member';

export default function members() {
  const [members, setMembers] = useState([]);
  const [newMemberName, setNewMemberName] = useState('');
  console.log(members);
  useEffect(() => {
    getMembers().then((res) => {
      setMembers(res);
    });
  }, []);

  return (
    <div>
      members
      <div>
      {
        members.map((member: Member) => {
          return (
            <div key={member.get_id()}>
              {member.get_name()}
            </div>
            );
          })
      }
      </div>
      <input type="text" value={newMemberName} onChange={(event) => setNewMemberName(event.target.value)} />
      <button type="button" onClick={() => postMember(newMemberName)}>Add</button>
    </div>
  );
}
