// Create Components
CREATE 
  (frontend:Component {name: 'Frontend', type: 'Presentation'}),
  (api_gateway:Component {name: 'API Gateway', type: 'Infrastructure'}),
  (auth:Component {name: 'Authentication', type: 'Security'}),
  (user_mgmt:Component {name: 'User Management', type: 'Data'}),
  (course_mgmt:Component {name: 'Course Management', type: 'Data'}),
  (messaging:Component {name: 'Messaging', type: 'Communication'}),
  (analytics:Component {name: 'Analytics', type: 'Processing'}),
  (storage:Component {name: 'File Storage', type: 'Infrastructure'}),
  (monitoring:Component {name: 'Monitoring', type: 'Operations'});

// Create Dependencies
MATCH 
  (f:Component {name: 'Frontend'}),
  (a:Component {name: 'API Gateway'}),
  (au:Component {name: 'Authentication'}),
  (u:Component {name: 'User Management'}),
  (c:Component {name: 'Course Management'}),
  (m:Component {name: 'Messaging'}),
  (an:Component {name: 'Analytics'}),
  (s:Component {name: 'File Storage'}),
  (mo:Component {name: 'Monitoring'})

CREATE
  (f)-[:DEPENDS_ON]->(a),
  (a)-[:DEPENDS_ON]->(au),
  (a)-[:DEPENDS_ON]->(u),
  (a)-[:DEPENDS_ON]->(c),
  (u)-[:DEPENDS_ON]->(s),
  (c)-[:DEPENDS_ON]->(s),
  (m)-[:DEPENDS_ON]->(a),
  (an)-[:DEPENDS_ON]->(c),
  (an)-[:DEPENDS_ON]->(u),
  (mo)-[:DEPENDS_ON]->(a),
  (mo)-[:DEPENDS_ON]->(u),
  (mo)-[:DEPENDS_ON]->(c);