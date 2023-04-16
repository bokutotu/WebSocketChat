export class Member {
  id: string;
  name: string;

  constructor(id: string, name: string) { 
    this.id = id;
    this.name = name;
  };

  public get_id() {
    return this.id;
  }

  public get_name() {
    return this.name;
  }
}
