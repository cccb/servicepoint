using ServicePoint;

var connection = new Connection("127.0.0.1:2342");
var clear = Command.Clear();
connection.Send(clear);
