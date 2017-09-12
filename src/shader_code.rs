pub const VERTEX_SHADER_CODE: &'static [u8] = b"
#version 400
precision mediump float;

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;

out vec3 Position;
out vec3 Normal;
out vec2 TexCoord;

uniform mat4 ViewModelMatrix;
uniform mat3 NormalMatrix;
uniform mat4 Projection;
uniform mat4 PVM;
void main()
{
  gl_PointSize = 100.0;
  TexCoord = VertexTexCoord;
  Normal = VertexNormal;
  Position = vec3(PVM*vec4(VertexPosition, 1.0));
  gl_Position = PVM*vec4(VertexPosition, 1.0);
}
\0";
// normalize(NormalMatrix * VertexNormal);
// Position = vec3(ModelViewMatrix * vec4(VertexPosition, 1.0));

pub const FRAGMENT_SHADER_CODE: &'static [u8] = b"
#version 400

precision mediump float;

in vec3 Position;
in vec3 Normal;
in vec2 TexCoord;

uniform sampler2D Tex1;
layout (location = 0) out vec4 FragColor;
void main()
{
  vec4 texColor = texture(Tex1, TexCoord);
  FragColor = texColor;
}
\0";