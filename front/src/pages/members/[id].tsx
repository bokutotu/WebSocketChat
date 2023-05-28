import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';

import { Member } from '../../types/member';
import { deleteMember, getMember } from '../../features/api';

export default function Member() {
  const router = useRouter();
  const { id } = router.query;
  
  const [member, setMember] = useState<Member | null>(null);
  useEffect(() => {
    if (id) {
      getMember(id as string).then(response => {
        setMember(response.getContent());
      });
    }
  }, []);

  return (
    <>
      <h1>Member {member?.id}</h1>
      <div>{member?.name}</div>
      <button onClick={() => deleteMember(id as string)}>削除</button>
    </>
  );
}
