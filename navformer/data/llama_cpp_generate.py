import json
import random
import re
import subprocess

from jsonschema import validate, ValidationError, SchemaError

# JSON schema
schema = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "properties": {
        "command": {
            "type": "string"
        },
        "actions": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["NAVIGATE", "HALT", "RETRIEVE", "DEPOSIT", "INTERACT", "REPORT"]
                    },
                    "description": {
                        "type": "string"
                    },
                    "intensity": {
                        "type": "number",
                        "minimum": 0,
                        "maximum": 1
                    },
                    "purpose": {
                        "type": "string"
                    },
                    "target": {
                        "type": ["string", "null"]
                    },
                    "value": {
                        "type": ["number", "string"]
                    }
                },
                "required": ["action", "description", "purpose"],
                "additionalProperties": False
            }
        }
    },
    "required": ["command", "actions"],
    "additionalProperties": False
}

# Define the robot's name
name = "Higgs"

# List of roles
roles = [
    "Aerial Photography Drone",
    "Agricultural Drone",
    "Aquaculture Monitoring System",
    "Archaeological Exploration Drone",
    "Art Restoration Assistant",
    "Artificial Intelligence Research Assistant",
    "Automated Inventory Management Drone",
    "Automated Librarian Assistant",
    "Automated Tutoring System",
    "Biological Sample Collection Drone",
    "Building Inspection Drone",
    "Cargo Transport Drone",
    "Cleaning Robot",
    "Climate Data Collection Drone",
    "Construction Drone",
    "Construction Site Survey Drone",
    "Crisis Management Drone",
    "Culinary Assistant Robot",
    "Customer Service Robot",
    "Delivery Drone",
    "Ecosystem Monitoring Drone",
    "Educational Assistant Robot",
    "Elderly Care Robot",
    "Electric Grid Inspection Drone",
    "Environmental Monitoring Drone",
    "Event Filming Drone",
    "Farm Management Drone",
    "Firefighting Drone",
    "Forensic Investigation Drone",
    "Freight Logistics Drone",
    "Gardening Drone",
    "Geological Survey Drone",
    "Glacial Monitoring Drone",
    "Golf Course Management Drone",
    "Healthcare Companion Robot",
    "Historical Site Preservation Drone",
    "Home Automation Integrator",
    "House Painting Drone",
    "Industrial Inspection Drone",
    "Journalism Drone",
    "Jungle Exploration Drone",
    "Lawn Mowing Robot",
    "Library Assistant Drone",
    "Livestock Management Drone",
    "Maintenance Drone",
    "Maritime Surveillance Drone",
    "Medical Consultation Robot",
    "Medical Delivery Drone",
    "Meteorological Research Drone",
    "Mine Detection Drone",
    "Mining Operation Drone",
    "Ocean Cleanup Drone",
    "Oceanographic Research Drone",
    "Oil Spill Monitoring Drone",
    "Orbital Debris Tracking Drone",
    "Organic Farming Assistant Drone",
    "Park Wildlife Monitoring Drone",
    "Personal Assistant Robot",
    "Personal Fitness Coach Robot",
    "Pest Control Drone",
    "Pipeline Inspection Drone",
    "Planetary Geology Drone",
    "Plant Pollination Drone",
    "Pollution Tracking Drone",
    "Port Security Drone",
    "Power Line Inspection Drone",
    "Precision Agriculture Drone",
    "Professional Photography Drone",
    "Public Health Monitoring Drone",
    "Public Transport Coordination Drone",
    "Radiation Monitoring Drone",
    "Real Estate Photography Drone",
    "Recreational Activity Assistant Robot",
    "Recycling Facility Drone",
    "Reforestation Drone",
    "Remote Sensing Drone",
    "Renewable Energy Inspection Drone",
    "Research and Development Assistant Robot",
    "Retail Inventory Drone",
    "River Monitoring Drone",
    "Road Maintenance Drone",
    "Robotic Nursing Assistant",
    "Search and Rescue Assistant",
    "Seismic Activity Monitoring Drone",
    "Smart City Coordinator Drone",
    "Smart Farming Coordinator Drone",
    "Smog Analysis Drone",
    "Snow Removal Robot",
    "Soil Analysis Drone",
    "Solar Panel Cleaning Drone",
    "Space Exploration Drone",
    "Sports Analytics Drone",
    "Sports Training Assistant Robot",
    "Storm Chaser Drone",
    "Street Cleaning Robot",
    "Structural Health Monitoring Drone",
    "Subterranean Mapping Drone",
    "Supply Chain Logistics Drone",
    "Surveying Drone",
    "Thermal Imaging Inspection Drone",
    "Traffic Congestion Analysis Drone",
    "Traffic Monitoring Drone",
    "Tunnel Inspection Drone",
    "Tutoring Robot",
    "Underwater Archaeology Drone",
    "Underwater Infrastructure Inspection Drone",
    "Underwater Survey Drone",
    "Urban Development Planning Drone",
    "Urban Farming Assistant Drone",
    "Urban Green Space Management Drone",
    "Utility Infrastructure Inspection Drone",
    "Veterinary Assistance Drone",
    "Volcanic Activity Monitoring Drone",
    "Waste Management Drone",
    "Water Conservation Drone",
    "Water Quality Monitoring Drone",
    "Weather Forecasting Drone",
    "Wildlife Conservation Drone",
    "Wind Turbine Inspection Drone",
    "Winery Management Drone"
]


# Function to validate JSON object against the schema
def validate_json(data, schema):
    try:
        validate(instance=data, schema=schema)
        return True  # The JSON data is valid
    except ValidationError as e:
        print("🔴 Given JSON data is invalid.")
        print("ValidationError:", e.message)
        return False  # The JSON data is invalid
    except SchemaError as e:
        print("🔴 JSON Schema is invalid.")
        print("SchemaError:", e.message)
        return False  # The schema itself is invalid


def save_valid_json(json_data, file_path):
    with open(file_path, 'a') as f:  # Append mode
        f.write(json_data + "\n\n")  # Write the valid JSON string plus a newline


def find_and_print_valid_json_objects(content):
    # List to hold valid JSON objects
    valid_json_objects = []

    # Temporary variable to hold potential JSON strings
    potential_json = ""
    in_json = False

    for char in content:
        if char == "{" and not in_json:
            # Possible start of a JSON object
            potential_json = char
            in_json = True
        elif in_json:
            potential_json += char
            if char == "}":
                # Possible end of a JSON object
                try:
                    # Attempt to parse the potential JSON string
                    obj = json.loads(potential_json)
                    valid_json_objects.append(obj)
                    # Reset for the next JSON object
                    potential_json = ""
                    in_json = False
                except json.JSONDecodeError:
                    # If it fails, keep looking in the string
                    continue

    # Loop through the potential JSON objects
    for obj in valid_json_objects:
        json_data = json.dumps(obj, indent=2)
        print(json_data)  # Print the JSON data for review

        # Validate the JSON object against the schema
        if validate_json(obj, schema):
            print("🟢 Given JSON data is valid.")
            # Save the valid JSON object
            save_valid_json(json_data, valid_json_file_path)


# Define a file path where you want to save the valid JSON objects
valid_json_file_path = 'valid_json_objects.txt'

number_of_iterations = 1000

for _ in range(number_of_iterations):
    # Select a random role from the list
    role = random.choice(roles)

    print(role);
    print("\n\n")

    # Define the prompt template with placeholders for the name and role
    prompt_template = """[INST]
    ROLE: {role}
    TASK: Generate JSON command objects for "{name}", the {role}, to execute tasks relevant to its role. Strictly adhere to the schema.

    SCHEMA INSTRUCTIONS:
    - "command": Initiate every command with "Hello {name}," adapting the task description to fit "{name}'s" role using clear, everyday language. To ensure a mix of command lengths:
    
    For short commands, focus on the essence of the task, suitable for rapid execution or simple tasks. Example: "Check the front door."
    For longer commands, incorporate additional specifics, such as "target," "value," and any relevant context or conditions that necessitate a detailed instruction. Example: "Navigate to the storage room, retrieve the spare projector, and set it up in Conference Room B for the 3 PM presentation."
    Specify the desired command length (short or long) based on the task's complexity or urgency, emphasizing practicality and directness for short commands and detailed clarity for longer commands. Adjust command detail level accordingly.
    
    - "actions": Enumerate the actions "{name}" will carry out, with details as follows:
      - "action": Specify the action type from "NAVIGATE", "HALT", "RETRIEVE", "DEPOSIT", "INTERACT".
      - "description": Provides a detailed explanation of the action's purpose and execution details, enhancing clarity and operational context.
      - "intensity": (If not applicable, OMIT this field) Sets action intensity on a 0 (minimum) to 1 (maximum) scale.
      - "purpose": Clarify the purpose of the action.
      - "target": (If not applicable, OMIT this field) Designate the object or location of interaction.
      - "value": (If not applicable, OMIT this field) Detail any supplementary specifics related to the action, either as a number or a string.

    REQUIREMENTS:
    - Only allow action types from "NAVIGATE", "HALT", "RETRIEVE", "DEPOSIT", "INTERACT".
    - Ensure no living being is harmed. Prioritize safety and well-being in all tasks and actions.
    - If "intensity", "target" or "value" do not apply to an action, those fields should be completely omitted from the action object. Do NOT fill these fields with placeholders such as empty strings, None, or null.
    - The final output must align 100% with this schema, showcasing "{name}"'s function as a {role}.
    - Each sequence of actions should conclude with a 'REPORT' action, which will detail task outcomes and confirm completion according to the instructions.
    - Alternate between simple and complex commands, so the AI model can generate a diverse range of JSON objects.

    SAMPLE COMMANDS:
    {{
        "command": "Hello {name}, commence the street cleaning operation by first navigating to Main Street, collecting all plastic debris.",
        "actions": [
            {{
                "action": "NAVIGATE",
                "description": "Navigate towards Main Street at a moderate speed to begin the street cleaning operation.",
                "intensity": 0.8,
                "purpose": "to_approach_cleaning_area",
                "target": "Main Street"
            }},
            {{
                "action": "INTERACT",
                "description": "Collect all plastic debris using onboard tools, ensuring thorough cleanup and safe handling of materials.",
                "intensity": 0.6,
                "purpose": "to_collect_plastic_debris",
                "target": "Main Street",
                "value": "plastic"
            }},
            {{
                "action": "REPORT",
                "description": "Report back to the central system with details of the cleanup, including the amount of plastic collected and the status of the cleaning operation.",
                "intensity": 0.5,
                "purpose": "to_confirm_cleanup_completion"
            }}
        ]
    }}

    {{
        "command": "Hello {name}, initiate emergency medical aid by retrieving the first aid kit from Storage Room 3 and delivering it to the site of the accident in the Courtyard.",
        "actions": [
            {{
                "action": "RETRIEVE",
                "description": "Retrieve the first aid kit from Storage Room 3, ensuring swift and secure handling.",
                "intensity": 0.7,
                "purpose": "to_collect_first_aid_kit",
                "target": "Storage Room 3",
                "value": "first aid kit"
            }},
            {{
                "action": "NAVIGATE",
                "description": "Navigate quickly to the Courtyard to deliver the first aid kit to the accident site, optimizing the fastest route for emergency response.",
                "intensity": 0.9,
                "purpose": "to_deliver_first_aid",
                "target": "Courtyard"
            }},
            {{
                "action": "REPORT",
                "description": "Send a detailed report on the delivery of the first aid kit, including time of delivery and any obstacles encountered, to confirm the completion of the emergency aid operation.",
                "intensity": 0.5,
                "purpose": "to_confirm_emergency_aid_completion"
            }}
        ]
    }}

    Generate JSON command objects for tasks, strictly following the schema and focusing on "{name}"'s role as a {role}.
    [/INST]"""

    # Use the format method to replace placeholders with the actual name and role
    prompt = prompt_template.format(name=name, role=role)

    executable_path = './llama.cpp/build/bin/main'
    model_path = './llama.cpp/models/mistral-7b-instruct-v0.2.Q4_K_M.gguf'

    command = [
        executable_path,
        '-m', model_path,
        '-n', '4096',
        '--repeat_penalty', '1.0',
        '--temp', '1.0',
        '--top-p', '1.0',
        '--ctx-size', '8000',
        '-p', prompt,
        '-ngl', '32'
    ]

    try:
        with open('temp.txt', 'w') as outfile:
            subprocess.run(command, stdout=outfile, check=True, text=True)
    except subprocess.CalledProcessError as e:
        print(f"Command execution failed with error: {e}")
        continue

    try:
        with open('temp.txt', 'r') as file:
            content = file.read()
            cleaned_content = re.sub(r'\[INST\].*?\[/INST\]', '', content, flags=re.DOTALL)
            trimmed_content = cleaned_content.strip()
            find_and_print_valid_json_objects(trimmed_content)
    except Exception as e:
        print(f"An error occurred: {e}")