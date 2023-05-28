import { useEffect, useState } from 'react';
import Link from 'next/link';

import { getMembers, postMember } from '../../features/api';

import { Member } from '../../types/member';

export default function members() {
  const [ members, setMembers ] = useState<Member[]>([]);
  const [ newMemberName, setNewMemberName ] = useState('');
  console.log(members);
  useEffect(() => {
    getMembers().then(response => {
      const members: Member[] = response.getContent();
      if (members) setMembers(members);
    });
  }, []);

  return (
    <div>
      members
      <div>
      {
        members.map((member: Member) => {
          const l: string = `/members/${member.id}`
          return (
            <Link href={l}>
              <div key={member.id}>
                {member.name}
              </div>
            </Link>
          );
          })
      }
      </div>
      <input type="text" value={newMemberName} onChange={(event) => setNewMemberName(event.target.value)} />
      <button type="button" onClick={() => {
        postMember(newMemberName).then(response => {
          const member: Member | null = response.getContent();
          if (member) {
            setMembers([...members, member]);
          } else {
            console.log('error');
          }
        });
        
      }}>Add</button>
    </div>
  );
}
