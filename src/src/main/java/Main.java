import org.json.simple.JSONArray;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;
import org.json.simple.parser.ParseException;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;

import static spark.Spark.*;

public class Main {


    public static void main(String[] args) {

        port(34843);
//todo log
        get("/search/:topic", (req,res)->{
            return search(req.params(":topic"));
        });
        get("/lookup/:id", (req,res)->{
            return lookup(req.params(":id"));
        });
        get("/buy/:topic", (req,res)->{
            return "buy, "+ req.params(":id");
        });

        get("/hello", (req, res) -> "Hello World");


    }
    public static JSONObject search(String topic) throws IOException {
        topic = topic.replace("-"," ");

        BufferedReader csvReader = new BufferedReader(new FileReader("inventory.csv"));
        String row;
        boolean contains = false;
        JSONObject ob = new JSONObject();
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            if(data[4].equals(topic)){
                contains=true;
                ob.put(data[0],Integer.valueOf(data[1]));
            }
        }

        csvReader.close();
        if (!contains){
            ob.put("message","no book under the topic is found");
        }
        return ob;
    }


    public static JSONObject lookup(String id) throws IOException {
        BufferedReader csvReader = new BufferedReader(new FileReader("inventory.csv"));
        String row;
        boolean contains = false;
        JSONObject ob1 = new JSONObject();
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            if(data[1].equals(id)){
                contains=true;
                ob1.put("name",data[0]);
                ob1.put("id",Integer.valueOf(data[1]));
                ob1.put("stock",Integer.valueOf(data[2]));
                ob1.put("price",data[3]);
                ob1.put("topic",data[4]);
                break;
            }
        }

        csvReader.close();
        if (!contains){
            ob1.put("message","no book under the id is found");
        }
        return ob1;
    }

}