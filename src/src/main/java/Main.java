import org.json.simple.JSONArray;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;
import org.json.simple.parser.ParseException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import sun.rmi.runtime.Log;

import java.io.*;

import static spark.Spark.*;

public class Main {

    private static Logger logger = LoggerFactory.getLogger(Main.class);

    public static void main(String[] args) {

        port(34843);
//todo log
        //todo At the beginning of each run, the Catalog log should
        // include one or more entries recording the initial state of
        // the catalog database (e.g. how many books were intially inserted)
        get("/search/:topic", (req,res)->{
            return search(req.params(":topic"));
        });
        get("/lookup/:id", (req,res)->{
            logger.info("look up for id");
            logger.info("id"+req.params(":id"));

            return lookup(req.params(":id"));
        });
        put("/buy/:id", (req,res)->{
            return buy(req.params(":id"));
        });
        get("/query/:id", (req,res)->{
            return querybuy(req.params(":id"));
        });



    }

    private static String  querybuy(String id) throws IOException {
        JSONObject info = lookup(id);
        logger.info("stock is "+info.get("stock").toString());
        if (Integer.parseInt(info.get("stock").toString())>1){
            return "true";
        } return "false";

    }

    private static JSONObject  buy(String id) throws IOException {


        BufferedReader csvReader = new BufferedReader(new FileReader("inventory.csv"));

        String row;
        boolean contains = false;
        String text ="";
        JSONObject ob = new JSONObject();
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            if(data[1].equals(id)){
                contains=true;
                text+= data[0]+","+data[1]+","+(Integer.valueOf(data[2])-1)+","+data[3]+","+data[4]+","+"\n";
                ob.put(data[0],(Integer.valueOf(data[2])-1));
            }else {
                text+=row+"\n";
            }

        }
        csvReader.close();

        FileWriter writer = new FileWriter("inventory.csv");
        writer.write(text);
        writer.close();

        if (!contains){
            ob.put("message","no book under the topic is found");
        }
        return ob;

    }

    public static JSONObject search(String topic) throws IOException {
        //todo sql
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
        //todo sql

        BufferedReader csvReader = new BufferedReader(new FileReader("../inventory.csv"));
        logger.info("look up for id"+ id);
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