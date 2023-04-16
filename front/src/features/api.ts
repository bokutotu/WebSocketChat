import axios from 'axios';

import { Member } from '../types/member';

axios.defaults.baseURL = 'http://localhost:1919';

// axiosを使用してAPIを叩く
// localhost:3000/membersにGETリクエストを送る
export async function getMembers() {
  try {
    const res = await axios.get('members', {withCredentials: false });
    console.log(res.data);
    return res.data.map(
      (member: any) => { 
        let id = member[0];
        let name = member[1];
        return new Member(id, name);
      }
    );
  } catch (error) {
    console.log(error);
  }
}

// axiosを使用してAPIを叩く
// localhost:3000/membersにPOSTリクエストを送る
export async function postMember(name: string) {
  try {
    const res = await axios.post('members', name );
    return res.data as Member;
  } catch (error) {
    console.log(error);
  }
}
