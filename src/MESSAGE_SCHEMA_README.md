# Message Documentation Schema

JSON Schema for documenting messages in the Oxide messaging framework.

## Files

- **message_schema.json** - Schema definition
- **message_example.yaml** - Example usage

## Example

```yaml
message_name: "sensor_telemetry"
protocol: "tcp"
packetizer: "json"
hardware_interface: "eth0"  # Optional
base_node: "sensor_controller"
whitelist_nodes:
  - "telemetry_processor"
  - "data_logger"
blacklist_nodes:
  - "legacy_system"
server: "tcp://127.0.0.1:5555"
message_type: "telemetry"
```

## Validation

```python
import json, yaml
from jsonschema import validate

with open('src/message_schema.json') as f:
    schema = json.load(f)
with open('your_message.yaml') as f:
    message_doc = yaml.safe_load(f)
    
validate(instance=message_doc, schema=schema)
```
