import axios, { AxiosResponse } from 'axios';

import { Member } from '../types/member';

axios.defaults.baseURL = 'http://localhost:1919';

export class Response<T> {
  status: number;
  content: T | null;

  constructor(status: number, content: T | null) {
    this.status = status;
    this.content = content;
  }

  isOk(): boolean {
    return this.status === 200;
  }

  public getContent(): T | null {
    return this.content;
  }
}

// axiosを使用してAPIを叩く
// localhost:3000/membersにGETリクエストを送る
export async function getMembers() {
  try {
    const res = await axios.get('members', {withCredentials: false });
    return new Response(res.status, res.data);
  } catch (error: any) {
    return new Response(error.response.status, null);
  }
}

// axiosを用いてAPIを叩く
// localhost:3000/members/idにGETリクエストを送る
// idはメンバーのID
// 例：localhost:3000/members/1
// 1はメンバーのID
export async function getMember(id: string) {
  try {
    const res = await axios.get(`members/${id}`, {withCredentials: false });
    return new Response(res.status, res.data);
  } catch (error: any) {
    return new Response(error.response.status, null);
  }
}

// axiosを使用してAPIを叩く
// localhost:3000/membersにPOSTリクエストを送る
export async function postMember(name: string) {
  try {
    const res = await axios.post('members', { name }, {withCredentials: false });
    return new Response(res.status, res.data);
  } catch (error: any) {
    return new Response(error.response.status, null);
  }
}

// axiosを使用してAPIを叩く
// localhost:3000/members/idにDeleteリクエストを送る
// idはメンバーのID
export async function deleteMember(id: string) {
  try {
    const res = await axios.delete(`members/${id}`, {withCredentials: false });
    return new Response(res.status, res.data);
  } catch (error: any) {
    return new Response(error.response.status, null);
  }
}
