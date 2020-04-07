import org.json.simple.JSONObject;

import java.io.*;
import java.nio.channels.FileChannel;
import java.nio.channels.FileLock;
import java.util.logging.FileHandler;
import java.util.logging.Logger;
import java.util.logging.SimpleFormatter;

import static java.lang.Thread.sleep;
import static spark.Spark.*;


public class Main {
    private final static Logger logger = Logger.getLogger(Main.class.getName());
    private static FileHandler fh;

    public static void main(String[] args) throws IOException {

        fh = new FileHandler("CATALOG.log");
        logger.addHandler(fh);
        SimpleFormatter formatter = new SimpleFormatter();
        fh.setFormatter(formatter);

        // the following statement is used to log any messages

        logger.info("Catalog running ");
        setuplog();
        port(34842);

        get("/search/:topic", (req,res)->{
            return search(req.params(":topic"));
        });
        get("/lookup/:id", (req,res)->{
            return lookup(req.params(":id"));
        });
        put("/buy/:id", (req,res)->{
            return buy(req.params(":id"));
        });
        get("/query/:id", (req,res)->{
            return querybuy(req.params(":id"));
        });



    }
// it records what is in the database, inventory of how many counts are there for each book.
    private static void setuplog() throws IOException {
        logger.info("======================================= ");

        BufferedReader csvReader = new BufferedReader(new FileReader("inventory.csv"));
        String row;
        boolean contains = false;
        JSONObject ob = new JSONObject();
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            logger.info("book id "+data[1]+" has "+data[2]+" in stock");
        }

        csvReader.close();
        logger.info("======================================= ");

    }

    private static String  querybuy(String id) throws IOException {
        // it first runs the look up to find the entry if it exists
        JSONObject info = lookup(id);
        if (info.containsKey("message")){
            return "false";
        }
        logger.info("stock for "+id+" is "+info.get("stock").toString());
        //if the book has any stock. If they do then return a string of true to the order server.
        if (Integer.parseInt(info.get("stock").toString())>0){
            return "true";
        }
        return "false";

    }
//Then order server sends the buy request as put
    private static String   buy(String id) throws IOException, InterruptedException {
        logger.info("Buying "+id);
// it takes a lock that locks the csv file aka the database
        RandomAccessFile file = new RandomAccessFile("inventory.csv", "rw");
        FileChannel channel = file.getChannel();
        FileLock lock = channel.tryLock();
        while(lock==null){
            sleep(1000);
            lock = channel.tryLock();
        }

        BufferedReader csvReader = new BufferedReader(new FileReader("inventory.csv"));

        String row;
        boolean contains = false;
        String text ="";
        String re = "";
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            if(data[1].equals(id)){
                //decrease the count
                contains=true;
                text+= data[0]+","+data[1]+","+(Integer.valueOf(data[2])-1)+","+data[3]+","+data[4]+","+"\n";
                re = data[0] +"---"+(Integer.valueOf(data[2])-1);
            }else {
                text+=row+"\n";
            }

        }
        csvReader.close();
//write new database
        FileWriter writer = new FileWriter("inventory.csv");
        writer.write(text);
        logger.info("Writting "+id+" after buy");
        writer.close();

        lock.release();

        file.close();
        channel.close();

        if (!contains){
            logger.info("doesnt not have "+id);
            re="message"+"no book under the topic is found";
        }
        return re;

    }

    //it takes the topic and find the matching topic from the csv file
    public static JSONObject search(String topic) throws IOException {
        logger.info("Searching for "+topic);
// For the topic name i replaced space with a dash since it would be better to identify and include from the http call.
        topic = topic.replace("-"," ");

        BufferedReader csvReader = new BufferedReader(new FileReader("inventory.csv"));
        String row;
        boolean contains = false;
        JSONObject ob = new JSONObject();
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            // If the topic colum matches it adds corresponsive info to a jason object and then return the object.
            if(data[4].equals(topic)){
                contains=true;
                ob.put(data[0],Integer.valueOf(data[1]));
            }
        }
//If none is found it return a message json object that nothing is found.
        csvReader.close();
        if (!contains){
            ob.put("message","no book under the topic is found");
        }
        return ob;
    }


  //  it reads the csv file and find the entry with matching id and similar with search it return all the details
    public static JSONObject lookup(String id) {
        BufferedReader csvReader = null;
        boolean contains = false;
        JSONObject ob1 = new JSONObject();

        try {
            csvReader = new BufferedReader(new FileReader("inventory.csv"));

        logger.info("look up for id"+ id);
        String row;
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            if(data[1].equals(id)){
                contains=true;
                ob1.put("name",data[0]);
                ob1.put("id",Integer.valueOf(data[1]));
                ob1.put("stock",Integer.valueOf(data[2]));
                ob1.put("price",data[3]);
                ob1.put("topic",data[4]);
                //The loop breaks when a matching id is found since id should be unique so it saves time.
                break;
            }
        }
            csvReader.close();
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }

        if (!contains){
            ob1.put("message","no book under the id is found");
        }
        return ob1;

    }

}